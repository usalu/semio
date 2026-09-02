//! ➗️ Independent `serde_json` producer/reader for the complete mathematical JSON carrier:
//! `{graph, geometry, equation}`.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &["change-coefficient", "change-graph-directed", "connect-nodes", "disconnect-nodes", "insert-point", "move-point", "remove-point", "replace-graph", "replace-points", "update-graph-algorithm"];

/// 🌱️ A deterministic complete carrier with non-trivial graph, geometry and equation state.
pub fn build_seed() -> Value {
    json!({
        "graph": {
            "directed": true,
            "nodes": [
                {"id": "a", "label": "A", "x": 10.0, "y": 20.0},
                {"id": "b", "label": "B", "x": 30.0, "y": 40.0},
                {"id": "c", "label": "C", "x": 50.0, "y": 60.0}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}],
            "algorithm": "topo",
            "algorithmSeed": null
        },
        "geometry": {"points": [{"x": 1.0, "y": 2.0}, {"x": 3.0, "y": 4.0}, {"x": 5.0, "y": 6.0}]},
        "equation": {
            "expr": {
                "label": 0,
                "kind": "add",
                "terms": [
                    {"label": 1, "kind": "mul", "factors": [
                        {"label": 2, "kind": "rational", "numer": "3", "denom": "4"},
                        {"label": 3, "kind": "symbol", "name": "x"}
                    ]},
                    {"label": 4, "kind": "integer", "lexeme": "2"}
                ]
            },
            "nextLabel": 5
        }
    })
}

pub fn arrange(_kind: &str, doc: &Value) -> Value {
    doc.clone()
}

/// ✍️ Applies one deterministic carrier-level edit without using the production mutation engine.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "change-coefficient" => {
            let node = doc
                .get_mut("equation")
                .and_then(|e| e.get_mut("expr"))
                .and_then(|e| e.get_mut("terms"))
                .and_then(Value::as_array_mut)
                .and_then(|terms| terms.first_mut())
                .and_then(|term| term.get_mut("factors"))
                .and_then(Value::as_array_mut)
                .and_then(|factors| factors.first_mut())
                .and_then(Value::as_object_mut)
                .ok_or_else(|| "the seed carries a rational coefficient at expr.terms[0].factors[0]".to_string())?;
            node.insert("numer".to_string(), Value::String("7".to_string()));
            node.insert("denom".to_string(), Value::String("5".to_string()));
        }
        "change-graph-directed" => doc["graph"]["directed"] = Value::Bool(false),
        "connect-nodes" => doc["graph"]["edges"].as_array_mut().ok_or("graph.edges must be an array")?.push(json!({"id": "e2", "source": "b", "target": "c"})),
        "disconnect-nodes" => {
            doc["graph"]["edges"].as_array_mut().ok_or("graph.edges must be an array")?.retain(|edge| edge["id"] != "e1");
        }
        "insert-point" => doc["geometry"]["points"].as_array_mut().ok_or("geometry.points must be an array")?.insert(1, json!({"x": 2.0, "y": 3.0})),
        "move-point" => doc["geometry"]["points"][1] = json!({"x": 33.0, "y": 44.0}),
        "remove-point" => {
            doc["geometry"]["points"].as_array_mut().ok_or("geometry.points must be an array")?.remove(1);
        }
        "replace-graph" => {
            doc["graph"] = json!({
                "directed": false,
                "nodes": [{"id": "z", "label": "Z", "x": 70.0, "y": 80.0}],
                "edges": [],
                "algorithm": "bfs",
                "algorithmSeed": "z"
            });
        }
        "replace-points" => doc["geometry"]["points"] = json!([{"x": -1.0, "y": -2.0}, {"x": 8.0, "y": 13.0}]),
        "update-graph-algorithm" => {
            doc["graph"]["algorithm"] = Value::String("dijkstra".into());
            doc["graph"]["algorithmSeed"] = Value::String("a".into());
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

/// 📄️ Canonical semantic projection of the complete foreign carrier.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["graph", "geometry", "equation"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    Ok(Value::Object(out))
}
