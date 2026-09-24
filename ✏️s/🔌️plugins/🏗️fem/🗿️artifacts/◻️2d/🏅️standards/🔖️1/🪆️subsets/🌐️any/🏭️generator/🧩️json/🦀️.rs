//! 🏗️ The `fem2d@1/any` non-geometric mutation vocabulary, expressed over this subset's own JSON
//! carrier and read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.
//!
//! Why this exists: the two mesh oracles already registered for this subset (`three-fem2d-mesh-reader`,
//! `manifold-fem2d-mesh-measure`) read the STL/OBJ export, so they witness GEOMETRY. A material's
//! Young's modulus, a support's restrained DOFs, a load case's self-weight flag and the analysis
//! settings do not move a single triangle — which is why 26 of this subset's 29 kinds were recorded
//! `-uncarried` against those two.
//!
//! But this subset's JSON export is not a stub. Unlike its csv/md/txt leaves, which wrap the DSL text
//! in a single blob, `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json` emits
//! `dsl::ToValue::to_value(snapshot)` — the real structured tree, every `Fem2dSnapshot` field. So all
//! nine arrays are carrier-level facts and a JSON reader witnesses every one of the 26.
//!
//! This is the same shape as the accepted `quick-xml`/svg and `burntsushi-csv`/mathematical readers:
//! the judge is a third-party implementation of the CARRIER, and nothing here predicts its answer.

use json::JsonValue;

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
    "replace-node",
    "replace-load",
    "change-load-case-name",
    "replace-combination",
];

/// 🌱️ A deterministic seed carrying at least TWO of every collection, because `delete-*` and
/// `replace-*` are only observable when the collection does not empty to nothing, and a corpus whose
/// mutations are not observable is not evidence. Field spelling follows the snapshot's value-codec
/// contract: `#[value(rename_all = "camelCase")]` on every record, `#[value(tag = "kind")]` on the
/// `FemElement` and `FemLoad` enums, and `FemDof` unrenamed (so `"Tx"`, not `"tx"`).
pub fn build_seed() -> JsonValue {
    literal(r#"{
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
    }"#)
}

fn literal(text: &str) -> JsonValue {
    json::parse(text).expect("a carrier literal is valid JSON")
}

fn array<'a>(value: &'a mut JsonValue, key: &str) -> Result<&'a mut Vec<JsonValue>, String> {
    match &mut value[key] {
        JsonValue::Array(items) => Ok(items),
        _ => Err(format!("{key} is not an array")),
    }
}

/// 🌾️ ARRANGEMENT — every kind's precondition already holds in the seed, which carries two of each
/// collection and two loads in `lc1`. Kept as an explicit identity so the generator's shape matches
/// the other subsets' and a future kind that DOES need a precondition has an obvious home.
pub fn arrange(_kind: &str, doc: &JsonValue) -> JsonValue {
    doc.clone()
}

/// ✍️ The forward mutation, as an edit to the JSON carrier.
pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {
    let mut doc = doc.clone();
    match kind {
        "create-node" => array(&mut doc, "nodes")?.push(literal(r#"{"id": "n4", "x": 0.0, "y": 3.0}"#)),
        "delete-node" => {
            array(&mut doc, "nodes")?.retain(|n| n["id"] != "n3");
        }
        "create-element" => array(&mut doc, "elements")?.push(literal(r#"{"kind": "bar", "id": "e3", "start": "n1", "end": "n3", "materialId": "m1", "sectionId": "s1"}"#)),
        "delete-element" => {
            array(&mut doc, "elements")?.retain(|e| e["id"] != "e2");
        }
        "replace-element" => {
            let elements = array(&mut doc, "elements")?;
            elements[1] = literal(r#"{"kind": "bar", "id": "e2", "start": "n2", "end": "n3", "materialId": "m1", "sectionId": "s1"}"#);
        }
        "create-material" => array(&mut doc, "materials")?.push(literal(r#"{"id": "m3", "name": "GL24h", "e": 11500000000.0, "nu": 0.2, "rho": 420.0}"#)),
        "delete-material" => {
            array(&mut doc, "materials")?.retain(|m| m["id"] != "m2");
        }
        "replace-material" => {
            let materials = array(&mut doc, "materials")?;
            materials[0] = literal(r#"{"id": "m1", "name": "S355", "e": 210000000000.0, "nu": 0.3, "rho": 7850.0}"#);
        }
        "create-section" => array(&mut doc, "sections")?.push(literal(r#"{"id": "s3", "name": "HEB200", "area": 0.00781, "iy": 0.0000570}"#)),
        "delete-section" => {
            array(&mut doc, "sections")?.retain(|s| s["id"] != "s2");
        }
        "replace-section" => {
            let sections = array(&mut doc, "sections")?;
            sections[0] = literal(r#"{"id": "s1", "name": "IPE220", "area": 0.00334, "iy": 0.0000272}"#);
        }
        "create-support" => array(&mut doc, "supports")?.push(literal(r#"{"id": "sup3", "nodeId": "n3", "fixed": ["Tx"]}"#)),
        "delete-support" => {
            array(&mut doc, "supports")?.retain(|s| s["id"] != "sup2");
        }
        "replace-support" => {
            let supports = array(&mut doc, "supports")?;
            supports[0] = literal(r#"{"id": "sup1", "nodeId": "n1", "fixed": ["Tx", "Ty"]}"#);
        }
        "create-load-case" => array(&mut doc, "loadCases")?.push(literal(r#"{"id": "lc3", "name": "wind", "loads": [], "selfWeight": false}"#)),
        "delete-load-case" => {
            array(&mut doc, "loadCases")?.retain(|c| c["id"] != "lc2");
        }
        "add-load" => {
            let cases = array(&mut doc, "loadCases")?;
            array(&mut cases[0], "loads")?.push(literal(r#"{"kind": "nodal", "id": "l4", "nodeId": "n2", "dof": "Tx", "value": 5000.0}"#));
        }
        "remove-load" => {
            let cases = array(&mut doc, "loadCases")?;
            array(&mut cases[0], "loads")?.retain(|l| l["id"] != "l2");
        }
        "change-load-case-self-weight" => {
            let cases = array(&mut doc, "loadCases")?;
            cases[0]["selfWeight"] = false.into();
        }
        "create-combination" => array(&mut doc, "combinations")?.push(literal(r#"{"id": "c3", "name": "ACC", "terms": [{"caseId": "lc1", "factor": 1.0}]}"#)),
        "delete-combination" => {
            array(&mut doc, "combinations")?.retain(|c| c["id"] != "c2");
        }
        "update-analysis-settings" => {
            doc["analysis"] = literal(r#"{"modalCount": 12, "bucklingCount": 8, "deformationScale": 250.0}"#);
        }
        "replace-node" => {
            let nodes = array(&mut doc, "nodes")?;
            nodes[0] = literal(r#"{"id": "n1", "x": 0.5, "y": 0.0}"#);
        }
        "replace-load" => {
            let cases = array(&mut doc, "loadCases")?;
            let loads = array(&mut cases[0], "loads")?;
            loads[0] = literal(r#"{"kind": "nodal", "id": "l1", "nodeId": "n3", "dof": "Ty", "value": -15000.0}"#);
        }
        "change-load-case-name" => {
            let cases = array(&mut doc, "loadCases")?;
            cases[0]["name"] = "permanent".into();
        }
        "replace-combination" => {
            let combinations = array(&mut doc, "combinations")?;
            combinations[0] = literal(r#"{"id": "c1", "name": "ULS 6.10b", "terms": [{"caseId": "lc1", "factor": 1.2}, {"caseId": "lc2", "factor": 1.5}]}"#);
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(doc)
}

/// 🔤️ Orders every object's keys, the committed carrier spelling; arrays keep their ORDER, so a
/// reordering is a difference, not a tie. Numbers keep their lexeme — no rounding, because a
/// tolerance here would silently accept a changed stiffness.
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

/// 📄️ The projection: the nine collections the kinds touch, canonicalised.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in ["nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis"] {
        out.insert(key, canonical(&parsed[key]));
    }
    Ok(JsonValue::Object(out))
}
