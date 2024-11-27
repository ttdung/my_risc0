// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use serde_json::json;
use jsonschema::{Draft, JSONSchema};
use json_validate_core::Outputs;
use risc0_zkvm::{
    guest::env,
};

fn main() {
    let (datastr, schemastr): (String, String) = env::read();

    let d : serde_json::Value  = serde_json::from_str(&datastr).unwrap();
    let s : serde_json::Value  = serde_json::from_str(&schemastr).unwrap();

    let data = json!(&d);
    let schema = json!(&s);

    println!(
        "json data {}",
        data
    );
    println!(
        "json schema {}",
        schema
    );

    // Compile the schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

    // // Validate the data against the schema
    let result = compiled_schema.validate(&data);

    let mut rs : u32 = 0;
    // Check the validation result
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
            break;
        }
    } else {
        println!("JSON data is valid against the schema.");
        rs = 1;
    }

    let out = Outputs {
        result: rs,
    };

    env::commit(&out);

}
