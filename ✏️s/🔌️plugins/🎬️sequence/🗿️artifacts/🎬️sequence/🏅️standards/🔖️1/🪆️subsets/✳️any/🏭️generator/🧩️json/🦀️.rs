//! 🎬️ Independent producer and semantic reader for the complete sequence JSON carrier.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &["change-step-collapsed", "connect-steps", "disconnect-steps", "move-step"];

pub fn build_seed() -> Value {
    json!({
        "schema": "sequence.sequence",
        "steps": [
            {"id": "a", "kind": "state.set", "params": {"key": "counter"}, "x": 10.0, "y": 20.0, "slot": null, "collapsed": false},
            {"id": "b", "kind": "log.print", "params": {"message": "hello"}, "x": 30.0, "y": 40.0, "slot": null, "collapsed": false},
            {"id": "c", "kind": "state.get", "params": {"key": "counter"}, "x": 50.0, "y": 60.0, "slot": null, "collapsed": false}
        ],
        "edges": [{"id": "e1", "from": "a", "to": "b"}]
    })
}

pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "change-step-collapsed" => doc["steps"][1]["collapsed"] = Value::Bool(true),
        "connect-steps" => doc["edges"].as_array_mut().ok_or("edges must be an array")?.push(json!({"id": "e2", "from": "b", "to": "c"})),
        "disconnect-steps" => doc["edges"].as_array_mut().ok_or("edges must be an array")?.retain(|edge| edge["id"] != "e1"),
        "move-step" => {
            doc["steps"][1]["x"] = json!(130.0);
            doc["steps"][1]["y"] = json!(240.0);
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

fn canonical(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), canonical(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["schema", "steps", "edges"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    Ok(Value::Object(out))
}
