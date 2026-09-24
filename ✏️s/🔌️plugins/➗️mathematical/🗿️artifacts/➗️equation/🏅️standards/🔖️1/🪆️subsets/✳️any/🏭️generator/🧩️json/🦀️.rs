//! ➗️ Independent `json` (json-rust) producer/reader for the complete equation JSON carrier:
//! `{graph, geometry, equation}`.
//!
//! Every value is written from its JSON lexeme, so numbers keep the spelling the carrier commits.

use json::JsonValue;

pub const KINDS: &[&str] = &["change-coefficient"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("change-coefficient", "🎚️change-coefficient")];

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

/// 🌱️ A deterministic complete carrier with non-trivial graph, geometry and equation state.
pub fn build_seed() -> JsonValue {
    literal(
        r#"{
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
        }"#,
    )
}

/// ✍️ Applies one deterministic carrier-level edit without using the production mutation engine.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "change-coefficient" => {
            let coefficient = &mut doc["equation"]["expr"]["terms"][0]["factors"][0];
            if coefficient["kind"] != "rational" {
                return Err("the seed carries a rational coefficient at expr.terms[0].factors[0]".to_string());
            }
            coefficient["numer"] = "7".into();
            coefficient["denom"] = "5".into();
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 🔤️ Orders every object's keys, the committed carrier spelling.
pub fn canonical(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Object(object) => {
            let mut entries: Vec<(&str, &JsonValue)> = object.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let mut sorted = json::object::Object::with_capacity(entries.len());
            for (key, member) in entries {
                sorted.insert(key, canonical(member));
            }
            JsonValue::Object(sorted)
        }
        JsonValue::Array(items) => JsonValue::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

/// 🖨️ The committed file bytes of one carrier.
pub fn render(value: &JsonValue) -> String {
    format!("{}\n", canonical(value).pretty(2))
}

/// 📄️ Canonical semantic projection of the complete foreign carrier.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["graph", "geometry", "equation"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
