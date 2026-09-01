//! ➗️ The one `mathematical@1/any` kind that IS a fact about this subset's own carrier:
//! `change-coefficient`.
//!
//! The other nine uncarried kinds are not a reader problem and cannot be fixed here. This subset's
//! state splits three ways:
//!
//! * `equation: EquationSnapshot` is INLINE in `MathematicalSnapshot`, so it reaches the JSON export
//!   (`serde_json::to_value(snapshot)`) intact. `change-coefficient` edits a labelled node of that
//!   expression tree, so it is a carrier-level fact and this reader witnesses it.
//! * The graph's NODES reach the CSV export — `id,label,x,y` per node, and nothing else. That is why
//!   `csv-rfc4180-mathematical-1-mutate` covers exactly the five node kinds.
//! * The graph's EDGES, its `directed` flag, its `algorithm`, and the geometry's POINTS reach NO
//!   carrier at all. `MathematicalIntoCsv` is declared `IoFidelity::Lossy` and emits only nodes; the
//!   JSON export carries `notation`/`results`/`computed` as `ArtifactChild` handles whose
//!   `local_owner` is `#[serde(skip)]`, so the scene behind them is never serialised.
//!
//! So `connect-nodes`, `disconnect-nodes`, `change-graph-directed`, `update-graph-algorithm`,
//! `replace-graph`, `insert-point`, `move-point`, `remove-point` and `replace-points` are blocked on
//! EXPORT, not on oracles — the same category as `tiff::change-byte-order`. A reader cannot witness
//! what no carrier records.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &["change-coefficient"];

/// 🌱️ A deterministic snapshot whose equation is `3/4 * x + 2`, so a coefficient change is a single
/// labelled node's edit against a tree that has more than one node. The composed children are carried
/// as the handles they really are — `{childId, target}` — because that is exactly what
/// `serde_json::to_value` emits for them.
pub fn build_seed() -> Value {
    json!({
        "notation": {"childId": "mathematical-scene-seed", "target": {"artifactId": "mathematical-text", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "text"}}},
        "results": {"childId": "mathematical-scene-seed", "target": {"artifactId": "mathematical-table", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"}}},
        "computed": {"childId": "mathematical-scene-seed", "target": {"artifactId": "mathematical-value", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "value"}}},
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

/// ✍️ Rewrites the coefficient at label 2 from `3/4` to `7/5` — a `Rational` lexeme pair, never an
/// `f64`, matching `EquationNodeKind::Rational`'s own round-trip-exact representation.
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

/// 📄️ The projection: the inline equation tree, plus the composed-child HANDLES as they actually
/// serialise. The handles are projected deliberately — not to witness the scene behind them, which
/// they do not carry, but so that a change in which child a snapshot points at is still a difference.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["equation", "notation", "results", "computed"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    Ok(Value::Object(out))
}
