//! 🏗️ The `fem2d@1/any` non-geometric mutation vocabulary, expressed over this subset's own JSON
//! carrier and read back through `serde_json` — a third-party JSON implementation and nothing of ours.
//!
//! Why this exists: the two mesh oracles already registered for this subset (`three-fem2d-mesh-reader`,
//! `manifold-fem2d-mesh-measure`) read the STL/OBJ export, so they witness GEOMETRY. A material's
//! Young's modulus, a support's restrained DOFs, a load case's self-weight flag and the analysis
//! settings do not move a single triangle — which is why 22 of this subset's 25 kinds were recorded
//! `-uncarried` against those two.
//!
//! But this subset's JSON export is not a stub. Unlike its csv/md/txt leaves, which wrap the DSL text
//! in a single blob, `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json` emits
//! `serde_json::to_value(snapshot)` — the real structured tree, every `Fem2dSnapshot` field. So all
//! nine arrays are carrier-level facts and a JSON reader witnesses every one of the 22.
//!
//! This is the same shape as the accepted `quick-xml`/svg and `burntsushi-csv`/mathematical readers:
//! the judge is a third-party implementation of the CARRIER, and nothing here predicts its answer.

use serde_json::{json, Map, Value};

pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
];

/// 🌱️ A deterministic seed carrying at least TWO of every collection, because `delete-*` and
/// `replace-*` are only observable when the collection does not empty to nothing, and a corpus whose
/// mutations are not observable is not evidence. Field spelling follows the snapshot's own serde
/// contract: `#[serde(rename_all = "camelCase")]` on every record, `#[serde(tag = "kind")]` on the
/// `FemElement` and `FemLoad` enums, and `FemDof` unrenamed (so `"Tx"`, not `"tx"`).
pub fn build_seed() -> Value {
    json!({
        "nodes": [
            {"id": "n1", "x": 0.0, "y": 0.0},
            {"id": "n2", "x": 4.0, "y": 0.0},
            {"id": "n3", "x": 4.0, "y": 3.0}
        ],
        "elements": [
            {"kind": "bar", "id": "e1", "start": "n1", "end": "n2", "materialId": "m1", "sectionId": "s1"},
            {"kind": "beam", "id": "e2", "start": "n2", "end": "n3", "materialId": "m2", "sectionId": "s2"}
        ],
        "regions": [
            {"id": "r1", "name": "slab", "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]], "holes": [], "thickness": 0.2, "materialId": "m1", "meshSize": 0.5}
        ],
        "materials": [
            {"id": "m1", "name": "S235", "e": 210000000000.0, "nu": 0.3, "rho": 7850.0},
            {"id": "m2", "name": "C30/37", "e": 33000000000.0, "nu": 0.2, "rho": 2500.0}
        ],
        "sections": [
            {"id": "s1", "name": "IPE200", "area": 0.00285, "iy": 0.0000194},
            {"id": "s2", "name": "IPE300", "area": 0.00538, "iy": 0.0000836}
        ],
        "supports": [
            {"id": "sup1", "nodeId": "n1", "fixed": ["Tx", "Ty", "Rz"]},
            {"id": "sup2", "nodeId": "n2", "fixed": ["Ty"]}
        ],
        "loadCases": [
            {"id": "lc1", "name": "dead", "loads": [
                {"kind": "nodal", "id": "l1", "nodeId": "n3", "dof": "Ty", "value": -12000.0},
                {"kind": "memberUdl", "id": "l2", "elementId": "e1", "wx": 0.0, "wy": -3500.0}
            ], "selfWeight": true},
            {"id": "lc2", "name": "live", "loads": [
                {"kind": "area", "id": "l3", "regionId": "r1", "pressure": -2500.0}
            ], "selfWeight": false}
        ],
        "combinations": [
            {"id": "c1", "name": "ULS", "terms": [{"caseId": "lc1", "factor": 1.35}, {"caseId": "lc2", "factor": 1.5}]},
            {"id": "c2", "name": "SLS", "terms": [{"caseId": "lc1", "factor": 1.0}, {"caseId": "lc2", "factor": 1.0}]}
        ],
        "analysis": {"modalCount": 6, "bucklingCount": 4, "deformationScale": 100.0}
    })
}

fn array<'a>(doc: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    doc.get_mut(key).and_then(Value::as_array_mut).expect("the seed declares every collection")
}

/// 🌾️ ARRANGEMENT — every kind's precondition already holds in the seed, which carries two of each
/// collection and two loads in `lc1`. Kept as an explicit identity so the generator's shape matches
/// the other subsets' and a future kind that DOES need a precondition has an obvious home.
pub fn arrange(_kind: &str, doc: &Value) -> Value {
    doc.clone()
}

/// ✍️ The forward mutation, as an edit to the JSON carrier.
pub fn apply(kind: &str, doc: &Value) -> Result<Value, String> {
    let mut doc = doc.clone();
    match kind {
        "create-node" => array(&mut doc, "nodes").push(json!({"id": "n4", "x": 0.0, "y": 3.0})),
        "delete-node" => {
            array(&mut doc, "nodes").retain(|n| n.get("id").and_then(Value::as_str) != Some("n3"));
        }
        "create-element" => array(&mut doc, "elements").push(json!({"kind": "bar", "id": "e3", "start": "n1", "end": "n3", "materialId": "m1", "sectionId": "s1"})),
        "delete-element" => {
            array(&mut doc, "elements").retain(|e| e.get("id").and_then(Value::as_str) != Some("e2"));
        }
        "replace-element" => {
            let elements = array(&mut doc, "elements");
            elements[1] = json!({"kind": "bar", "id": "e2", "start": "n2", "end": "n3", "materialId": "m1", "sectionId": "s1"});
        }
        "create-material" => array(&mut doc, "materials").push(json!({"id": "m3", "name": "GL24h", "e": 11500000000.0, "nu": 0.2, "rho": 420.0})),
        "delete-material" => {
            array(&mut doc, "materials").retain(|m| m.get("id").and_then(Value::as_str) != Some("m2"));
        }
        "replace-material" => {
            let materials = array(&mut doc, "materials");
            materials[0] = json!({"id": "m1", "name": "S355", "e": 210000000000.0, "nu": 0.3, "rho": 7850.0});
        }
        "create-section" => array(&mut doc, "sections").push(json!({"id": "s3", "name": "HEB200", "area": 0.00781, "iy": 0.0000570})),
        "delete-section" => {
            array(&mut doc, "sections").retain(|s| s.get("id").and_then(Value::as_str) != Some("s2"));
        }
        "replace-section" => {
            let sections = array(&mut doc, "sections");
            sections[0] = json!({"id": "s1", "name": "IPE220", "area": 0.00334, "iy": 0.0000272});
        }
        "create-support" => array(&mut doc, "supports").push(json!({"id": "sup3", "nodeId": "n3", "fixed": ["Tx"]})),
        "delete-support" => {
            array(&mut doc, "supports").retain(|s| s.get("id").and_then(Value::as_str) != Some("sup2"));
        }
        "replace-support" => {
            let supports = array(&mut doc, "supports");
            supports[0] = json!({"id": "sup1", "nodeId": "n1", "fixed": ["Tx", "Ty"]});
        }
        "create-load-case" => array(&mut doc, "loadCases").push(json!({"id": "lc3", "name": "wind", "loads": [], "selfWeight": false})),
        "delete-load-case" => {
            array(&mut doc, "loadCases").retain(|c| c.get("id").and_then(Value::as_str) != Some("lc2"));
        }
        "add-load" => {
            let cases = array(&mut doc, "loadCases");
            cases[0].get_mut("loads").and_then(Value::as_array_mut).ok_or("lc1 has no loads array")?.push(json!({"kind": "nodal", "id": "l4", "nodeId": "n2", "dof": "Tx", "value": 5000.0}));
        }
        "remove-load" => {
            let cases = array(&mut doc, "loadCases");
            cases[0].get_mut("loads").and_then(Value::as_array_mut).ok_or("lc1 has no loads array")?.retain(|l| l.get("id").and_then(Value::as_str) != Some("l2"));
        }
        "change-load-case-self-weight" => {
            let cases = array(&mut doc, "loadCases");
            cases[0].as_object_mut().ok_or("lc1 is not an object")?.insert("selfWeight".to_string(), Value::Bool(false));
        }
        "create-combination" => array(&mut doc, "combinations").push(json!({"id": "c3", "name": "ACC", "terms": [{"caseId": "lc1", "factor": 1.0}]})),
        "delete-combination" => {
            array(&mut doc, "combinations").retain(|c| c.get("id").and_then(Value::as_str) != Some("c2"));
        }
        "update-analysis-settings" => {
            doc.as_object_mut().ok_or("document is not an object")?.insert("analysis".to_string(), json!({"modalCount": 12, "bucklingCount": 8, "deformationScale": 250.0}));
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 📄️ Canonicalises for comparison: object keys sorted, arrays left in ORDER (so a reordering is a
/// difference, not a tie). Numbers are compared as `serde_json` parsed them — no rounding, because a
/// tolerance here would silently accept a changed stiffness.
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

/// 📄️ The projection: the nine collections the 22 kinds touch, canonicalised.
pub fn project(bytes: &[u8]) -> Result<Value, String> {
    let parsed: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut out = Map::new();
    for key in ["nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis"] {
        out.insert(key.to_string(), canonical(parsed.get(key).unwrap_or(&Value::Null)));
    }
    Ok(Value::Object(out))
}
