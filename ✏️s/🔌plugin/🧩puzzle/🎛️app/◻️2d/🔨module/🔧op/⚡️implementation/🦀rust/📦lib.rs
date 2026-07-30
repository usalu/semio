//! ⚡ Puzzle 2d app — operation enum + laws (constitutional: op).

use puzzle_2d::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dProjection};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Puzzle2dEnvelope = store::DocumentEnvelope<Puzzle2dProjection, Puzzle2dOperation>;
pub type Puzzle2dStore = store::DocumentStore<Puzzle2dProjection, Puzzle2dOperation>;

// #region 🔖Collections
/// 🪪 Stable-id accessor shared by every id-keyed document collection entry.
trait Puzzle2dHasId {
    fn id(&self) -> &str;
}

impl Puzzle2dHasId for Puzzle2dNode {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle2dHasId for Puzzle2dEdge {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
/// already exists, else insert at the recorded index).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dNodesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle2dNode)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dEdgesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle2dEdge)>,
}

fn apply_puzzle2d_collection_diff<T: Puzzle2dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

fn puzzle2d_index_of<T: Puzzle2dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🩹 Sparse puzzle-2d diff over both id-keyed collections plus the scalar camera/meta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dDiff {
    /// 🌍 Whole-document replacement (example load, engine fill, layout); wins over every field below.
    pub document: Option<Puzzle2dProjection>,
    pub nodes: Puzzle2dNodesDiff,
    pub edges: Puzzle2dEdgesDiff,
    pub camera: Option<Puzzle2dCamera>,
    pub meta: Option<Puzzle2dMeta>,
}

fn puzzle2d_diff_absorb(diff: &mut Puzzle2dDiff, other: Puzzle2dDiff) {
    if other.document.is_some() {
        *diff = Puzzle2dDiff { document: other.document, ..Default::default() };
        return;
    }
    diff.nodes.removed.extend(other.nodes.removed);
    diff.nodes.set.extend(other.nodes.set);
    diff.edges.removed.extend(other.edges.removed);
    diff.edges.set.extend(other.edges.set);
    if other.camera.is_some() {
        diff.camera = other.camera;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Puzzle2dProjection> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dProjection) -> Puzzle2dProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_puzzle2d_collection_diff(&mut next.nodes, &self.nodes.removed, &self.nodes.set);
        apply_puzzle2d_collection_diff(&mut next.edges, &self.edges.removed, &self.edges.set);
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle2d_diff_absorb(self, other);
    }
}

/// 🧮 Puzzle-2d operation: id-keyed node/edge edits plus scalar camera/meta, each with a true inverse
/// computed from the pre-operation projection, and a whole-document replace for example loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Puzzle2dOperation {
    #[dsl(key = "setNode")]
    SetNode { index: usize, #[dsl(block)] node: Puzzle2dNode },
    #[dsl(key = "removeNode")]
    RemoveNode { id: String },
    #[dsl(key = "setEdge")]
    SetEdge { index: usize, #[dsl(block)] edge: Puzzle2dEdge },
    #[dsl(key = "removeEdge")]
    RemoveEdge { id: String },
    #[dsl(key = "setCamera")]
    SetCamera { #[dsl(block)] camera: Puzzle2dCamera },
    #[dsl(key = "setMeta")]
    SetMeta { #[dsl(block)] meta: Puzzle2dMeta },
    /// 🌍 Replaces the whole document (example import / reset / engine fill).
    #[dsl(key = "setDocument")]
    SetDocument { #[dsl(block)] document: Puzzle2dProjection },
}

fn puzzle2d_operation_diff(operation: &Puzzle2dOperation) -> Puzzle2dDiff {
    let mut diff = Puzzle2dDiff::default();
    match operation {
        Puzzle2dOperation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
        Puzzle2dOperation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
        Puzzle2dOperation::SetEdge { index, edge } => diff.edges.set.push((*index, edge.clone())),
        Puzzle2dOperation::RemoveEdge { id } => diff.edges.removed.push(id.clone()),
        Puzzle2dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
        Puzzle2dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Puzzle2dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Puzzle2dProjection> for Puzzle2dOperation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, _projection: &Puzzle2dProjection) -> Puzzle2dDiff {
        puzzle2d_operation_diff(self)
    }

    fn backwards(&self, projection: &Puzzle2dProjection) -> Vec<Self> {
        match self {
            Puzzle2dOperation::SetNode { node, .. } => match puzzle2d_index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Puzzle2dOperation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Puzzle2dOperation::RemoveNode { id: node.id.clone() }],
            },
            Puzzle2dOperation::RemoveNode { id } => puzzle2d_index_of(&projection.nodes, id).map(|index| vec![Puzzle2dOperation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Puzzle2dOperation::SetEdge { edge, .. } => match puzzle2d_index_of(&projection.edges, &edge.id) {
                Some(index) => vec![Puzzle2dOperation::SetEdge { index, edge: projection.edges[index].clone() }],
                None => vec![Puzzle2dOperation::RemoveEdge { id: edge.id.clone() }],
            },
            Puzzle2dOperation::RemoveEdge { id } => puzzle2d_index_of(&projection.edges, id).map(|index| vec![Puzzle2dOperation::SetEdge { index, edge: projection.edges[index].clone() }]).unwrap_or_default(),
            Puzzle2dOperation::SetCamera { .. } => vec![Puzzle2dOperation::SetCamera { camera: projection.camera.clone() }],
            Puzzle2dOperation::SetMeta { .. } => vec![Puzzle2dOperation::SetMeta { meta: projection.meta.clone() }],
            Puzzle2dOperation::SetDocument { .. } => vec![Puzzle2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖Operations

// #region 🔖ValueBridge
// 🌉 `puzzle-plugin`'s scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture (out of scope for this ticket — see
// `.repo/🎫/…/convertpuzzle2d3d5dtotypeddslderiveengine`). Bridging `Puzzle2dOperation`/`Puzzle2dDiff`
// onto that `Value` boundary too keeps `puzzle2d_document_delta_operations(&Value, &Value)` and the
// plugin's `DocumentApp::Projection = Value` compiling unchanged: `apply` serializes the typed
// payload back to JSON and splices it into the id-keyed array/field exactly like the pre-migration
// untyped operation did.
fn puzzle2d_value_item_id(item: &Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

/// 🩹 Replaces the id-matching entry in place, else inserts at `index` (clamped to the current
/// length) — matching `apply_puzzle2d_collection_diff`'s insert-at-recorded-index semantics on the
/// typed projection, so undo/redo restores the original array position on this `Value` bridge too.
fn puzzle2d_upsert_value_item(document: &mut Value, collection: &str, index: usize, item: Value) {
    let Some(object) = document.as_object_mut() else {
        return;
    };
    let array = object.entry(collection.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    let Some(array) = array.as_array_mut() else {
        return;
    };
    if let Some(id) = puzzle2d_value_item_id(&item).map(str::to_string) {
        if let Some(slot) = array.iter_mut().find(|entry| puzzle2d_value_item_id(entry) == Some(id.as_str())) {
            *slot = item;
            return;
        }
    }
    array.insert(index.min(array.len()), item);
}

fn puzzle2d_remove_value_item(document: &mut Value, collection: &str, id: &str) {
    if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
        array.retain(|entry| puzzle2d_value_item_id(entry) != Some(id));
    }
}

fn apply_puzzle2d_operation_to_value(document: &mut Value, operation: &Puzzle2dOperation) {
    match operation {
        Puzzle2dOperation::SetNode { index, node } => puzzle2d_upsert_value_item(document, "nodes", *index, serde_json::to_value(node).unwrap_or(Value::Null)),
        Puzzle2dOperation::RemoveNode { id } => puzzle2d_remove_value_item(document, "nodes", id),
        Puzzle2dOperation::SetEdge { index, edge } => puzzle2d_upsert_value_item(document, "edges", *index, serde_json::to_value(edge).unwrap_or(Value::Null)),
        Puzzle2dOperation::RemoveEdge { id } => puzzle2d_remove_value_item(document, "edges", id),
        Puzzle2dOperation::SetCamera { camera } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("camera".to_string(), serde_json::to_value(camera).unwrap_or(Value::Null));
            }
        }
        Puzzle2dOperation::SetMeta { meta } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        Puzzle2dOperation::SetDocument { document: next } => *document = serde_json::to_value(next).unwrap_or_else(|_| document.clone()),
    }
}

fn puzzle2d_value_collection<'a>(document: &'a Value, collection: &str) -> &'a [Value] {
    document.get(collection).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

fn puzzle2d_value_item_index<T: serde::de::DeserializeOwned>(document: &Value, collection: &str, id: &str) -> Option<(usize, T)> {
    let items = puzzle2d_value_collection(document, collection);
    let index = items.iter().position(|entry| puzzle2d_value_item_id(entry) == Some(id))?;
    serde_json::from_value(items[index].clone()).ok().map(|item| (index, item))
}

impl OperationDiff<Value> for Puzzle2dDiff {
    fn apply(&self, projection: &Value) -> Value {
        if let Some(document) = &self.document {
            return serde_json::to_value(document).unwrap_or_else(|_| projection.clone());
        }
        let mut next = projection.clone();
        for id in &self.nodes.removed {
            puzzle2d_remove_value_item(&mut next, "nodes", id);
        }
        for (index, node) in &self.nodes.set {
            puzzle2d_upsert_value_item(&mut next, "nodes", *index, serde_json::to_value(node).unwrap_or(Value::Null));
        }
        for id in &self.edges.removed {
            puzzle2d_remove_value_item(&mut next, "edges", id);
        }
        for (index, edge) in &self.edges.set {
            puzzle2d_upsert_value_item(&mut next, "edges", *index, serde_json::to_value(edge).unwrap_or(Value::Null));
        }
        if let Some(camera) = &self.camera {
            if let Some(object) = next.as_object_mut() {
                object.insert("camera".to_string(), serde_json::to_value(camera).unwrap_or(Value::Null));
            }
        }
        if let Some(meta) = &self.meta {
            if let Some(object) = next.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle2d_diff_absorb(self, other);
    }
}

impl Operation<Value> for Puzzle2dOperation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, _projection: &Value) -> Puzzle2dDiff {
        puzzle2d_operation_diff(self)
    }

    fn backwards(&self, projection: &Value) -> Vec<Self> {
        match self {
            Puzzle2dOperation::SetNode { node, .. } => match puzzle2d_value_item_index::<Puzzle2dNode>(projection, "nodes", &node.id) {
                Some((index, previous)) => vec![Puzzle2dOperation::SetNode { index, node: previous }],
                None => vec![Puzzle2dOperation::RemoveNode { id: node.id.clone() }],
            },
            Puzzle2dOperation::RemoveNode { id } => puzzle2d_value_item_index::<Puzzle2dNode>(projection, "nodes", id).map(|(index, previous)| vec![Puzzle2dOperation::SetNode { index, node: previous }]).unwrap_or_default(),
            Puzzle2dOperation::SetEdge { edge, .. } => match puzzle2d_value_item_index::<Puzzle2dEdge>(projection, "edges", &edge.id) {
                Some((index, previous)) => vec![Puzzle2dOperation::SetEdge { index, edge: previous }],
                None => vec![Puzzle2dOperation::RemoveEdge { id: edge.id.clone() }],
            },
            Puzzle2dOperation::RemoveEdge { id } => puzzle2d_value_item_index::<Puzzle2dEdge>(projection, "edges", id).map(|(index, previous)| vec![Puzzle2dOperation::SetEdge { index, edge: previous }]).unwrap_or_default(),
            Puzzle2dOperation::SetCamera { .. } => {
                let camera: Puzzle2dCamera = projection.get("camera").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle2dOperation::SetCamera { camera }]
            }
            Puzzle2dOperation::SetMeta { .. } => {
                let meta: Puzzle2dMeta = projection.get("meta").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle2dOperation::SetMeta { meta }]
            }
            Puzzle2dOperation::SetDocument { .. } => vec![Puzzle2dOperation::SetDocument { document: serde_json::from_value(projection.clone()).unwrap_or_default() }],
        }
    }
}

/// 🧮 Collects the sparse `set`/`removed` delta for one id-keyed `Value` array collection into typed
/// entries. Returns `false` (caller falls back to `SetDocument`) whenever an entry is missing an
/// `id` or fails to deserialize into `T` — the granular path only ever fires when it's exact.
fn puzzle2d_collect_value_collection_delta<T>(before: &[Value], after: &[Value], set: &mut Vec<(usize, T)>, removed: &mut Vec<String>) -> bool
where
    T: serde::de::DeserializeOwned,
{
    for (index, entry) in after.iter().enumerate() {
        let Some(id) = puzzle2d_value_item_id(entry) else {
            return false;
        };
        if before.iter().find(|candidate| puzzle2d_value_item_id(candidate) == Some(id)) != Some(entry) {
            let Ok(item) = serde_json::from_value::<T>(entry.clone()) else {
                return false;
            };
            set.push((index, item));
        }
    }
    for entry in before {
        let Some(id) = puzzle2d_value_item_id(entry) else {
            return false;
        };
        if !after.iter().any(|candidate| puzzle2d_value_item_id(candidate) == Some(id)) {
            removed.push(id.to_string());
        }
    }
    true
}

/// 🧮 Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// fixture JSON `puzzle-plugin` mutates). Node/edge arrays diff per element id; camera/meta become
/// `SetCamera`/`SetMeta`. Falls back to a single `SetDocument` whenever the granular replay would not
/// reproduce `after` exactly (reorders, id-less entries, malformed entries, unrecognized top-level
/// keys, schema changes) — so the emitted operations are always exact while staying granular for the
/// common edits.
pub fn puzzle2d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle2dOperation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle2dOperation::SetDocument { document: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };
    const KNOWN_KEYS: [&str; 5] = ["schema", "camera", "nodes", "edges", "meta"];
    if before_object.keys().chain(after_object.keys()).any(|key| !KNOWN_KEYS.contains(&key.as_str())) {
        return fallback(after);
    }
    if before_object.get("schema") != after_object.get("schema") {
        return fallback(after);
    }
    let mut operations = Vec::new();
    let before_nodes = before_object.get("nodes").and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
    let after_nodes = after_object.get("nodes").and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
    if before_nodes != after_nodes {
        let mut set = Vec::new();
        let mut removed = Vec::new();
        if !puzzle2d_collect_value_collection_delta::<Puzzle2dNode>(before_nodes, after_nodes, &mut set, &mut removed) {
            return fallback(after);
        }
        operations.extend(removed.into_iter().map(|id| Puzzle2dOperation::RemoveNode { id }));
        operations.extend(set.into_iter().map(|(index, node)| Puzzle2dOperation::SetNode { index, node }));
    }
    let before_edges = before_object.get("edges").and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
    let after_edges = after_object.get("edges").and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
    if before_edges != after_edges {
        let mut set = Vec::new();
        let mut removed = Vec::new();
        if !puzzle2d_collect_value_collection_delta::<Puzzle2dEdge>(before_edges, after_edges, &mut set, &mut removed) {
            return fallback(after);
        }
        operations.extend(removed.into_iter().map(|id| Puzzle2dOperation::RemoveEdge { id }));
        operations.extend(set.into_iter().map(|(index, edge)| Puzzle2dOperation::SetEdge { index, edge }));
    }
    let before_camera = before_object.get("camera");
    let after_camera = after_object.get("camera");
    if before_camera != after_camera {
        let Some(camera) = after_camera.and_then(|value| serde_json::from_value::<Puzzle2dCamera>(value.clone()).ok()) else {
            return fallback(after);
        };
        operations.push(Puzzle2dOperation::SetCamera { camera });
    }
    let before_meta = before_object.get("meta");
    let after_meta = after_object.get("meta");
    if before_meta != after_meta {
        let meta = match after_meta {
            Some(value) => match serde_json::from_value::<Puzzle2dMeta>(value.clone()) {
                Ok(meta) => meta,
                Err(_) => return fallback(after),
            },
            None => Puzzle2dMeta::default(),
        };
        operations.push(Puzzle2dOperation::SetMeta { meta });
    }
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle2d_operation_to_value(&mut replay, operation);
    }
    if &replay == after {
        operations
    } else {
        fallback(after)
    }
}

// #region 🔖PlayProjection
/// 🌱 `puzzle-plugin`'s `Puzzle2dPlayApp` predates the typed `Puzzle2dProjection` above and stays on
/// this ad-hoc `serde_json::Value` fixture shape for its hundreds of Value-manipulating scene-mutation
/// helpers (see `puzzle-plugin`'s own module docs) — out of scope to retrofit onto the typed struct.
/// This newtype exists only to satisfy `DocumentApp::Projection: store::DocumentDsl + store::DocumentPack`
/// post the repo-wide `store::DocumentDsl for serde_json::Value` bridge's removal (final DSL-syntax
/// convergence gate); `parse_dsl`/`print_dsl`/`encode_pack_with`/`decode_pack_with` all round-trip
/// straight through the still-standing `serde_json::Value` impls (JSON text / JSON-bridge pack
/// encoding respectively), same local-bridge shape as `compose`'s `KitSnapshot`. `Operation`/
/// `OperationDiff` delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Puzzle2dPlayProjection(pub Value);

impl store::DocumentDsl for Puzzle2dPlayProjection {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle2dPlayProjection).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle2dPlayProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.0.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Value::decode_pack_with(bytes, options).map(Puzzle2dPlayProjection)
    }
}

impl OperationDiff<Puzzle2dPlayProjection> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlayProjection) -> Puzzle2dPlayProjection {
        Puzzle2dPlayProjection(OperationDiff::<Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        puzzle2d_diff_absorb(self, other);
    }
}

impl Operation<Puzzle2dPlayProjection> for Puzzle2dOperation {
    type Diff = Puzzle2dDiff;

    fn diff(&self, projection: &Puzzle2dPlayProjection) -> Puzzle2dDiff {
        Operation::<Value>::diff(self, &projection.0)
    }

    fn backwards(&self, projection: &Puzzle2dPlayProjection) -> Vec<Self> {
        Operation::<Value>::backwards(self, &projection.0)
    }
}
// #endregion 🔖PlayProjection

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn puzzle2d_delta_ops_are_granular_and_round_trip() {
        use crate::{puzzle2d_document_delta_operations, Puzzle2dOperation};
        use puzzle_2d::PUZZLE_2D_SCHEMA;
        use serde_json::Value;
        use protocol::{Operation, OperationDiff};

        let before = json!({ "schema": PUZZLE_2D_SCHEMA, "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "n1", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
        // Move n2, add n3, remove n1, pan the camera — a disjoint mix of granular edits.
        let after = json!({ "schema": PUZZLE_2D_SCHEMA, "camera": { "x": 5.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "n2", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
        let operations = puzzle2d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle2dOperation::SetNode { .. })));
        assert!(!operations.iter().any(|operation| matches!(operation, Puzzle2dOperation::SetDocument { .. })), "granular delta must not fall back to whole-document replace here");
        // Forward replay (over the bare Value fixture, mirroring how `puzzle-plugin` applies these) reproduces
        // `after`, and each operation's backwards restores `before`.
        let mut forward = before.clone();
        let mut inverses = Vec::new();
        for operation in &operations {
            inverses.extend(Operation::<Value>::backwards(operation, &forward));
            forward = Operation::<Value>::diff(operation, &forward).apply(&forward);
        }
        assert_eq!(forward, after);
        for inverse in inverses.iter().rev() {
            forward = Operation::<Value>::diff(inverse, &forward).apply(&forward);
        }
        assert_eq!(forward, before, "backwards operations must restore the pre-edit document");
    }
}
//#endregion 🧪Tests
