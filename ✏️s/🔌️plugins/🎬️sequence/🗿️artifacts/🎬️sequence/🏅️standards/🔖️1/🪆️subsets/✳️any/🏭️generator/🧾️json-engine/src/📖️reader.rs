//! 🔍️ Projects or compares committed sequence JSON carriers through `serde_json`.

use std::process::exit;

use sequence_json::project;
use serde_json::json;

fn report(status: &str, measurements: serde_json::Value) -> String {
    json!({
        "schema": "semio.repository-test.probe-report/v2",
        "probe": "sequence-json-carrier",
        "probeVersion": "serde_json@1",
        "engine": {"family": "serde-json", "implementation": "serde_json 1 value tree", "version": "1"},
        "status": status,
        "durationMs": 0,
        "measurements": measurements
    })
    .to_string()
}

fn read(path: &str) -> Result<serde_json::Value, String> {
    project(&std::fs::read(path).map_err(|error| error.to_string())?)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.get(1).map(String::as_str) {
        Some("project") => read(args.get(2).expect("usage: reader project <file.json>")).map(|value| report("ok", value)),
        Some("compare") => {
            let left = read(args.get(2).expect("usage: reader compare <expected.json> <actual.json>"));
            let right = read(args.get(3).expect("usage: reader compare <expected.json> <actual.json>"));
            left.and_then(|left| right.map(|right| report("ok", json!({"equal": left == right, "expected": left, "actual": right}))))
        }
        _ => Err("usage: reader project <file.json> | reader compare <expected.json> <actual.json>".into()),
    };
    match result {
        Ok(report) => println!("{report}"),
        Err(error) => {
            println!("{}", report("failed", json!({"error": error})));
            exit(1);
        }
    }
}
