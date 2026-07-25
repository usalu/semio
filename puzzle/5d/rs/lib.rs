//! 👯 Puzzle 5d brush/fill precompute and document VCS on `vcs`.

//#region ⚠️ Errors
/// 🧯 Puzzle 5d precompute session errors — delegates entirely to `puzzle_3d`'s own precompute-session error.
#[derive(Debug, thiserror::Error)]
pub enum Puzzle5dError {
    #[error(transparent)]
    Puzzle3d(#[from] puzzle_3d::Puzzle3dError),
}
//#endregion ⚠️ Errors

//#region 🔖BrushEngine
pub use puzzle_3d::BrushPlacePayload;

pub struct Puzzle5dPrecomputeSession {
    inner: puzzle_3d::Puzzle3dPrecomputeSession,
}

impl Default for Puzzle5dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle5dPrecomputeSession {
    pub fn new() -> Self {
        Self { inner: puzzle_3d::Puzzle3dPrecomputeSession::new() }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle5dError> {
        Ok(self.inner.set_scene(json)?)
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.inner.register_mesh(url, positions, indices);
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.inner.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.inner.precompute_step(budget)
    }

    pub fn brush_candidates(&self, grip_full_id: &str) -> String {
        self.inner.brush_candidates(grip_full_id)
    }

    pub fn brush_preview_json(&self, grip_full_id: &str, candidate_index: usize) -> Option<String> {
        self.inner.brush_preview_json(grip_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> String {
        self.inner.fill_progress()
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_brush_placement_rust(payload_json)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(self.inner.apply_fill_count_rust(count)?)
    }
}
//#endregion 🔖BrushEngine

//#region 🔖KindCompatibility
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲 Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's `kindCompatibility` rows —
/// the single shared table both the 2D board and 3D world honor so brush/fill suggestions agree across projections.
pub fn puzzle5d_grip_kinds_compatible(source_kind: &str, target_kind: &str) -> bool {
    let Some(manifest) = mathematical_graph_manifest::manifest_by_id(PUZZLE5D_DEFAULT_MANIFEST_ID) else {
        return false;
    };
    manifest.kind_compatibility.iter().any(|row| {
        let source = row.get("source").and_then(|value| value.as_str());
        let target = row.get("target").and_then(|value| value.as_str());
        let bidirectional = row.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == Some(source_kind) && target == Some(target_kind)) || (bidirectional && source == Some(target_kind) && target == Some(source_kind))
    })
}
//#endregion 🔖KindCompatibility

// 🧩 Puzzle 5d document VCS on `vcs`: granular JSON-document operations over the bare document
// projection (parts keyed by id, camera + scalar fields) with a whole-document fallback, so
// disjoint edits converge instead of clobbering.
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(any(test, target_arch = "wasm32"))]
use vcs::{create_document_vcs_envelope, DocumentVcsCommand};
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const PUZZLE_5D_SCHEMA: &str = "puzzle.5d";

/// 🧩 The puzzle-5d projection is the bare document json (schema/camera/parts/…).
pub type Puzzle5dProjection = Value;
pub type Puzzle5dEnvelope = DocumentVcsEnvelope<Puzzle5dProjection, Puzzle5dOperation>;
pub type Puzzle5dStore = DocumentVcsStore<Puzzle5dProjection, Puzzle5dOperation>;

/// 🔧 One granular mutation of a JSON puzzle document. `UpsertItem`/`RemoveItem` address an element
/// of a top-level id-keyed array so disjoint edits converge; `SetField` writes a scalar/object field;
/// `ReplaceDocument` swaps the whole document (example load, engine fill, layout).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Puzzle5dOperation {
    UpsertItem { collection: String, item: Value },
    RemoveItem { collection: String, id: String },
    SetField { key: String, value: Value },
    ReplaceDocument { document: Value },
}

/// 🧮 An ordered list of granular operations replayed over the projection; coalesced edits concatenate.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Puzzle5dDiff {
    pub operations: Vec<Puzzle5dOperation>,
}

fn puzzle5d_item_id(item: &Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

fn apply_puzzle5d_operation(document: &mut Value, operation: &Puzzle5dOperation) {
    match operation {
        Puzzle5dOperation::UpsertItem { collection, item } => {
            let Some(object) = document.as_object_mut() else {
                return;
            };
            let array = object.entry(collection.clone()).or_insert_with(|| Value::Array(Vec::new()));
            let Some(array) = array.as_array_mut() else {
                return;
            };
            if let Some(id) = puzzle5d_item_id(item).map(str::to_string) {
                if let Some(slot) = array.iter_mut().find(|entry| puzzle5d_item_id(entry) == Some(id.as_str())) {
                    *slot = item.clone();
                    return;
                }
            }
            array.push(item.clone());
        }
        Puzzle5dOperation::RemoveItem { collection, id } => {
            if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
                array.retain(|entry| puzzle5d_item_id(entry) != Some(id.as_str()));
            }
        }
        Puzzle5dOperation::SetField { key, value } => {
            if let Some(object) = document.as_object_mut() {
                object.insert(key.clone(), value.clone());
            }
        }
        Puzzle5dOperation::ReplaceDocument { document: next } => *document = next.clone(),
    }
}

fn puzzle5d_find_item<'a>(document: &'a Value, collection: &str, id: &str) -> Option<&'a Value> {
    document.get(collection).and_then(|value| value.as_array()).and_then(|array| array.iter().find(|entry| puzzle5d_item_id(entry) == Some(id)))
}

impl OperationDiff<Puzzle5dProjection> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dProjection) -> Puzzle5dProjection {
        let mut next = projection.clone();
        for operation in &self.operations {
            apply_puzzle5d_operation(&mut next, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.operations.extend(other.operations);
    }
}

impl Operation<Puzzle5dProjection> for Puzzle5dOperation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, _projection: &Puzzle5dProjection) -> Puzzle5dDiff {
        Puzzle5dDiff { operations: vec![self.clone()] }
    }

    fn backwards(&self, projection: &Puzzle5dProjection) -> Vec<Self> {
        match self {
            Puzzle5dOperation::UpsertItem { collection, item } => {
                let id = puzzle5d_item_id(item).unwrap_or_default();
                match puzzle5d_find_item(projection, collection, id) {
                    Some(previous) => vec![Puzzle5dOperation::UpsertItem { collection: collection.clone(), item: previous.clone() }],
                    None => vec![Puzzle5dOperation::RemoveItem { collection: collection.clone(), id: id.to_string() }],
                }
            }
            Puzzle5dOperation::RemoveItem { collection, id } => match puzzle5d_find_item(projection, collection, id) {
                Some(previous) => vec![Puzzle5dOperation::UpsertItem { collection: collection.clone(), item: previous.clone() }],
                None => Vec::new(),
            },
            Puzzle5dOperation::SetField { key, .. } => vec![Puzzle5dOperation::SetField { key: key.clone(), value: projection.get(key).cloned().unwrap_or(Value::Null) }],
            Puzzle5dOperation::ReplaceDocument { .. } => vec![Puzzle5dOperation::ReplaceDocument { document: projection.clone() }],
        }
    }
}

fn puzzle5d_is_id_keyed_array(value: Option<&Value>) -> bool {
    value.and_then(|value| value.as_array()).is_some_and(|array| array.iter().all(|entry| puzzle5d_item_id(entry).is_some()))
}

fn puzzle5d_collect_collection_delta(collection: &str, before: &[Value], after: &[Value], operations: &mut Vec<Puzzle5dOperation>) {
    for entry in after {
        let id = puzzle5d_item_id(entry).unwrap_or_default();
        if before.iter().find(|candidate| puzzle5d_item_id(candidate) == Some(id)) != Some(entry) {
            operations.push(Puzzle5dOperation::UpsertItem { collection: collection.to_string(), item: entry.clone() });
        }
    }
    for entry in before {
        let id = puzzle5d_item_id(entry).unwrap_or_default();
        if !after.iter().any(|candidate| puzzle5d_item_id(candidate) == Some(id)) {
            operations.push(Puzzle5dOperation::RemoveItem { collection: collection.to_string(), id: id.to_string() });
        }
    }
}

/// 🧮 Computes the granular operation sequence turning `before` into `after`, falling back to a single
/// `ReplaceDocument` whenever the granular replay would not reproduce `after` exactly.
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dOperation> {
    if before == after {
        return Vec::new();
    }
    let operations = match (before.as_object(), after.as_object()) {
        (Some(before_object), Some(after_object)) => {
            let mut keys: Vec<&String> = before_object.keys().chain(after_object.keys()).collect();
            keys.sort();
            keys.dedup();
            let mut operations = Vec::new();
            for key in keys {
                let before_value = before_object.get(key);
                let after_value = after_object.get(key);
                if before_value == after_value {
                    continue;
                }
                match after_value {
                    Some(after_value) if puzzle5d_is_id_keyed_array(before_value) && puzzle5d_is_id_keyed_array(Some(after_value)) => {
                        let before_array = before_value.and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
                        puzzle5d_collect_collection_delta(key, before_array, after_value.as_array().map(Vec::as_slice).unwrap_or(&[]), &mut operations);
                    }
                    Some(after_value) => operations.push(Puzzle5dOperation::SetField { key: key.clone(), value: after_value.clone() }),
                    None => operations.push(Puzzle5dOperation::SetField { key: key.clone(), value: Value::Null }),
                }
            }
            operations
        }
        _ => vec![Puzzle5dOperation::ReplaceDocument { document: after.clone() }],
    };
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle5d_operation(&mut replay, operation);
    }
    if &replay == after {
        operations
    } else {
        vec![Puzzle5dOperation::ReplaceDocument { document: after.clone() }]
    }
}

pub fn empty_puzzle5d_projection() -> Value {
    serde_json::json!({
        "schema": PUZZLE_5D_SCHEMA,
        "parts": []
    })
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Puzzle5dDocumentVcs {
        store: RefCell<Puzzle5dStore>,
    }

    #[wasm_bindgen]
    impl Puzzle5dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Puzzle5dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Puzzle5dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Puzzle5dStore::new(envelope)
                }
                None => Puzzle5dStore::new(create_document_vcs_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_document_vcs_replays_granular_operations() {
        let mut store = Puzzle5dStore::new(create_document_vcs_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_projection(), None));
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Puzzle5dOperation::UpsertItem { collection: "parts".into(), item: serde_json::json!({ "id": "p1" }) }], description: None }).expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.get("parts").and_then(|value| value.as_array()).map(Vec::len), Some(1));
    }

    #[test]
    fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
        assert!(puzzle5d_grip_kinds_compatible("port", "port"));
        assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
    }
}
