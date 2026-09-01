//! 🔍️ The READER: projects a committed fem2d JSON carrier through `serde_json` and compares two of
//! them. Nothing here applies a mutation or predicts what one should produce.
//!
//! usage: reader project <file.json> | reader compare <expected.json> <actual.json>
use std::process::exit;

use fem2d_json::project;
use serde_json::json;

fn report(probe: &str, status: &str, measurements: serde_json::Value) -> String {
    json!({
        "schema": "semio.repository-test.probe-report/v2",
        "probe": probe,
        "probeVersion": "serde_json@1",
        "engine": {"family": "serde-json", "implementation": "serde_json 1 value tree", "version": "1"},
        "status": status,
        "durationMs": 0,
        "measurements": measurements,
    })
    .to_string()
}

fn read(path: &str) -> Result<serde_json::Value, String> {
    project(&std::fs::read(path).map_err(|error| error.to_string())?)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => match read(args.get(2).expect("usage: reader project <file.json>")) {
            Ok(value) => println!("{}", report("fem2d-json-project", "ok", value)),
            Err(error) => {
                println!("{}", report("fem2d-json-project", "failed", json!({"error": error})));
                exit(1);
            }
        },
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (read(expected), read(actual)) {
                (Ok(left), Ok(right)) => println!("{}", report("fem2d-json-compare", "ok", json!({"equal": left == right, "expected": left, "actual": right}))),
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("fem2d-json-compare", "failed", json!({"error": error})));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");
            exit(2);
        }
    }
}
