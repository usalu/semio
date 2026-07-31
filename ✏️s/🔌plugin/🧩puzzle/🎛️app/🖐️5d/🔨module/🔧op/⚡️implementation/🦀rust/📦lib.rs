//! ⚡ Puzzle 5d app — operation enum + laws (constitutional: op).

use puzzle_5d::{Puzzle5dFastener, Puzzle5dMeta, Puzzle5dPart, Puzzle5dProjection};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Puzzle5dEnvelope = store::DocumentEnvelope<Puzzle5dProjection, Puzzle5dOperation>;
pub type Puzzle5dStore = store::DocumentStore<Puzzle5dProjection, Puzzle5dOperation>;

// #region 🔖Collections
trait Puzzle5dHasId {
    fn id(&self) -> &str;
}
impl Puzzle5dHasId for Puzzle5dPart {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle5dHasId for Puzzle5dFastener {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPartsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle5dPart)>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dFastenersDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle5dFastener)>,
}

fn apply_puzzle5d_collection_diff<T: Puzzle5dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

fn puzzle5d_index_of<T: Puzzle5dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🩹 Sparse puzzle-5d diff over both id-keyed collections plus the scalar meta. Camera pose is
/// session-only app runtime state, never part of this diff — see `puzzle_5d_ui`'s `Puzzle5dRuntime`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDiff {
    /// 🌍 Whole-document replacement (example load, engine fill, layout); wins over every field below.
    pub document: Option<Puzzle5dProjection>,
    pub parts: Puzzle5dPartsDiff,
    pub fasteners: Puzzle5dFastenersDiff,
    pub meta: Option<Puzzle5dMeta>,
}

fn puzzle5d_diff_absorb(diff: &mut Puzzle5dDiff, other: Puzzle5dDiff) {
    if other.document.is_some() {
        *diff = Puzzle5dDiff { document: other.document, ..Default::default() };
        return;
    }
    diff.parts.removed.extend(other.parts.removed);
    diff.parts.set.extend(other.parts.set);
    diff.fasteners.removed.extend(other.fasteners.removed);
    diff.fasteners.set.extend(other.fasteners.set);
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Puzzle5dProjection> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dProjection) -> Puzzle5dProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_puzzle5d_collection_diff(&mut next.parts, &self.parts.removed, &self.parts.set);
        apply_puzzle5d_collection_diff(&mut next.fasteners, &self.fasteners.removed, &self.fasteners.set);
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle5d_diff_absorb(self, other);
    }
}

/// 🧮 Puzzle-5d operation: id-keyed part/fastener edits plus scalar meta, each with a true inverse
/// computed from the pre-operation projection, and a whole-document replace for example loads (also
/// the only path that changes `schema`/`domain`/`label`/`kindCatalogs`/`kindCompatibility` —
/// static/rare fields with no granular editor today). Camera pose is session-only app runtime state
/// (`ActionKind::View`), never a document operation — see `puzzle_5d_ui`'s `Puzzle5dRuntime`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Puzzle5dOperation {
    #[dsl(key = "setPart")]
    SetPart { index: usize, #[dsl(block)] part: Puzzle5dPart },
    #[dsl(key = "removePart")]
    RemovePart { id: String },
    #[dsl(key = "setFastener")]
    SetFastener { index: usize, #[dsl(block)] fastener: Puzzle5dFastener },
    #[dsl(key = "removeFastener")]
    RemoveFastener { id: String },
    #[dsl(key = "setMeta")]
    SetMeta { #[dsl(block)] meta: Puzzle5dMeta },
    /// 🌍 Replaces the whole document (example import / reset / engine fill).
    #[dsl(key = "setDocument")]
    SetDocument { #[dsl(block)] document: Puzzle5dProjection },
}

fn puzzle5d_operation_diff(operation: &Puzzle5dOperation) -> Puzzle5dDiff {
    let mut diff = Puzzle5dDiff::default();
    match operation {
        Puzzle5dOperation::SetPart { index, part } => diff.parts.set.push((*index, part.clone())),
        Puzzle5dOperation::RemovePart { id } => diff.parts.removed.push(id.clone()),
        Puzzle5dOperation::SetFastener { index, fastener } => diff.fasteners.set.push((*index, fastener.clone())),
        Puzzle5dOperation::RemoveFastener { id } => diff.fasteners.removed.push(id.clone()),
        Puzzle5dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Puzzle5dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Puzzle5dProjection> for Puzzle5dOperation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, _projection: &Puzzle5dProjection) -> Puzzle5dDiff {
        puzzle5d_operation_diff(self)
    }

    fn backwards(&self, projection: &Puzzle5dProjection) -> Vec<Self> {
        match self {
            Puzzle5dOperation::SetPart { part, .. } => match puzzle5d_index_of(&projection.parts, &part.id) {
                Some(index) => vec![Puzzle5dOperation::SetPart { index, part: projection.parts[index].clone() }],
                None => vec![Puzzle5dOperation::RemovePart { id: part.id.clone() }],
            },
            Puzzle5dOperation::RemovePart { id } => puzzle5d_index_of(&projection.parts, id).map(|index| vec![Puzzle5dOperation::SetPart { index, part: projection.parts[index].clone() }]).unwrap_or_default(),
            Puzzle5dOperation::SetFastener { fastener, .. } => match puzzle5d_index_of(&projection.fasteners, &fastener.id) {
                Some(index) => vec![Puzzle5dOperation::SetFastener { index, fastener: projection.fasteners[index].clone() }],
                None => vec![Puzzle5dOperation::RemoveFastener { id: fastener.id.clone() }],
            },
            Puzzle5dOperation::RemoveFastener { id } => puzzle5d_index_of(&projection.fasteners, id).map(|index| vec![Puzzle5dOperation::SetFastener { index, fastener: projection.fasteners[index].clone() }]).unwrap_or_default(),
            Puzzle5dOperation::SetMeta { .. } => vec![Puzzle5dOperation::SetMeta { meta: projection.meta.clone() }],
            Puzzle5dOperation::SetDocument { .. } => vec![Puzzle5dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖Operations

// #region 🔖ValueBridge
// 🌉 `puzzle-plugin`'s scene-mutation helpers predate this typed projection and stay on a bare
// `serde_json::Value` scratch fixture (out of scope for this ticket). Bridging `Puzzle5dOperation`/
// `Puzzle5dDiff` onto that `Value` boundary too keeps `puzzle5d_document_delta_operations` and the
// plugin's `DocumentApp::Projection = Value` compiling unchanged — mirrors `puzzle_2d`/`puzzle_3d`'s bridge.
fn puzzle5d_value_item_id(item: &Value) -> Option<&str> {
    item.get("id").and_then(|value| value.as_str())
}

fn puzzle5d_upsert_value_item(document: &mut Value, collection: &str, index: usize, item: Value) {
    let Some(object) = document.as_object_mut() else {
        return;
    };
    let array = object.entry(collection.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    let Some(array) = array.as_array_mut() else {
        return;
    };
    if let Some(id) = puzzle5d_value_item_id(&item).map(str::to_string) {
        if let Some(slot) = array.iter_mut().find(|entry| puzzle5d_value_item_id(entry) == Some(id.as_str())) {
            *slot = item;
            return;
        }
    }
    array.insert(index.min(array.len()), item);
}

fn puzzle5d_remove_value_item(document: &mut Value, collection: &str, id: &str) {
    if let Some(array) = document.get_mut(collection).and_then(|value| value.as_array_mut()) {
        array.retain(|entry| puzzle5d_value_item_id(entry) != Some(id));
    }
}

fn apply_puzzle5d_operation_to_value(document: &mut Value, operation: &Puzzle5dOperation) {
    match operation {
        Puzzle5dOperation::SetPart { index, part } => puzzle5d_upsert_value_item(document, "parts", *index, serde_json::to_value(part).unwrap_or(Value::Null)),
        Puzzle5dOperation::RemovePart { id } => puzzle5d_remove_value_item(document, "parts", id),
        Puzzle5dOperation::SetFastener { index, fastener } => puzzle5d_upsert_value_item(document, "fasteners", *index, serde_json::to_value(fastener).unwrap_or(Value::Null)),
        Puzzle5dOperation::RemoveFastener { id } => puzzle5d_remove_value_item(document, "fasteners", id),
        Puzzle5dOperation::SetMeta { meta } => {
            if let Some(object) = document.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        Puzzle5dOperation::SetDocument { document: next } => *document = serde_json::to_value(next).unwrap_or_else(|_| document.clone()),
    }
}

fn puzzle5d_value_collection<'a>(document: &'a Value, collection: &str) -> &'a [Value] {
    document.get(collection).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

fn puzzle5d_value_item_index<T: serde::de::DeserializeOwned>(document: &Value, collection: &str, id: &str) -> Option<(usize, T)> {
    let items = puzzle5d_value_collection(document, collection);
    let index = items.iter().position(|entry| puzzle5d_value_item_id(entry) == Some(id))?;
    serde_json::from_value(items[index].clone()).ok().map(|item| (index, item))
}

impl OperationDiff<Value> for Puzzle5dDiff {
    fn apply(&self, projection: &Value) -> Value {
        if let Some(document) = &self.document {
            return serde_json::to_value(document).unwrap_or_else(|_| projection.clone());
        }
        let mut next = projection.clone();
        for id in &self.parts.removed {
            puzzle5d_remove_value_item(&mut next, "parts", id);
        }
        for (index, part) in &self.parts.set {
            puzzle5d_upsert_value_item(&mut next, "parts", *index, serde_json::to_value(part).unwrap_or(Value::Null));
        }
        for id in &self.fasteners.removed {
            puzzle5d_remove_value_item(&mut next, "fasteners", id);
        }
        for (index, fastener) in &self.fasteners.set {
            puzzle5d_upsert_value_item(&mut next, "fasteners", *index, serde_json::to_value(fastener).unwrap_or(Value::Null));
        }
        if let Some(meta) = &self.meta {
            if let Some(object) = next.as_object_mut() {
                object.insert("meta".to_string(), serde_json::to_value(meta).unwrap_or(Value::Null));
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle5d_diff_absorb(self, other);
    }
}

impl Operation<Value> for Puzzle5dOperation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, _projection: &Value) -> Puzzle5dDiff {
        puzzle5d_operation_diff(self)
    }

    fn backwards(&self, projection: &Value) -> Vec<Self> {
        match self {
            Puzzle5dOperation::SetPart { part, .. } => match puzzle5d_value_item_index::<Puzzle5dPart>(projection, "parts", &part.id) {
                Some((index, previous)) => vec![Puzzle5dOperation::SetPart { index, part: previous }],
                None => vec![Puzzle5dOperation::RemovePart { id: part.id.clone() }],
            },
            Puzzle5dOperation::RemovePart { id } => puzzle5d_value_item_index::<Puzzle5dPart>(projection, "parts", id).map(|(index, previous)| vec![Puzzle5dOperation::SetPart { index, part: previous }]).unwrap_or_default(),
            Puzzle5dOperation::SetFastener { fastener, .. } => match puzzle5d_value_item_index::<Puzzle5dFastener>(projection, "fasteners", &fastener.id) {
                Some((index, previous)) => vec![Puzzle5dOperation::SetFastener { index, fastener: previous }],
                None => vec![Puzzle5dOperation::RemoveFastener { id: fastener.id.clone() }],
            },
            Puzzle5dOperation::RemoveFastener { id } => {
                puzzle5d_value_item_index::<Puzzle5dFastener>(projection, "fasteners", id).map(|(index, previous)| vec![Puzzle5dOperation::SetFastener { index, fastener: previous }]).unwrap_or_default()
            }
            Puzzle5dOperation::SetMeta { .. } => {
                let meta: Puzzle5dMeta = projection.get("meta").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                vec![Puzzle5dOperation::SetMeta { meta }]
            }
            Puzzle5dOperation::SetDocument { .. } => vec![Puzzle5dOperation::SetDocument { document: serde_json::from_value(projection.clone()).unwrap_or_default() }],
        }
    }
}

/// 🧮 Collects the sparse `set`/`removed` delta for one id-keyed `Value` array collection into typed
/// entries. Returns `false` (caller falls back to `SetDocument`) whenever an entry is missing an
/// `id` or fails to deserialize into `T`.
fn puzzle5d_collect_value_collection_delta<T>(before: &[Value], after: &[Value], set: &mut Vec<(usize, T)>, removed: &mut Vec<String>) -> bool
where
    T: serde::de::DeserializeOwned,
{
    let before_by_id: std::collections::HashMap<&str, &Value> = before.iter().filter_map(|entry| puzzle5d_value_item_id(entry).map(|id| (id, entry))).collect();
    let mut after_ids: std::collections::HashSet<&str> = std::collections::HashSet::with_capacity(after.len());
    for (index, entry) in after.iter().enumerate() {
        let Some(id) = puzzle5d_value_item_id(entry) else {
            return false;
        };
        after_ids.insert(id);
        if before_by_id.get(id).copied() != Some(entry) {
            let Ok(item) = serde_json::from_value::<T>(entry.clone()) else {
                return false;
            };
            set.push((index, item));
        }
    }
    for entry in before {
        let Some(id) = puzzle5d_value_item_id(entry) else {
            return false;
        };
        if !after_ids.contains(id) {
            removed.push(id.to_string());
        }
    }
    true
}

/// 🧮 Computes the granular typed operation sequence turning `before` into `after` (both the bare
/// document JSON `puzzle-plugin` mutates). Falls back to a single `SetDocument` whenever the granular
/// replay would not reproduce `after` exactly, or whenever `schema`/`domain`/`label`/`kindCatalogs`/
/// `kindCompatibility` changed (no granular editor for those today — see `Puzzle5dOperation`'s doc).
pub fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dOperation> {
    if before == after {
        return Vec::new();
    }
    let fallback = |after: &Value| vec![Puzzle5dOperation::SetDocument { document: serde_json::from_value(after.clone()).unwrap_or_default() }];
    let (Some(before_object), Some(after_object)) = (before.as_object(), after.as_object()) else {
        return fallback(after);
    };
    const KNOWN_KEYS: [&str; 8] = ["schema", "domain", "label", "meta", "kindCatalogs", "kindCompatibility", "parts", "fasteners"];
    if before_object.keys().chain(after_object.keys()).any(|key| !KNOWN_KEYS.contains(&key.as_str())) {
        return fallback(after);
    }
    for exact_key in ["schema", "domain", "label", "kindCatalogs", "kindCompatibility"] {
        if before_object.get(exact_key) != after_object.get(exact_key) {
            return fallback(after);
        }
    }
    let mut operations = Vec::new();
    macro_rules! collect_collection {
        ($key:literal, $set_op:expr, $remove_op:expr, $ty:ty) => {{
            let before_items = before_object.get($key).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
            let after_items = after_object.get($key).and_then(|value| value.as_array()).map(Vec::as_slice).unwrap_or(&[]);
            if before_items != after_items {
                let mut set = Vec::new();
                let mut removed = Vec::new();
                if !puzzle5d_collect_value_collection_delta::<$ty>(before_items, after_items, &mut set, &mut removed) {
                    return fallback(after);
                }
                operations.extend(removed.into_iter().map($remove_op));
                operations.extend(set.into_iter().map($set_op));
            }
        }};
    }
    collect_collection!("parts", |(index, part)| Puzzle5dOperation::SetPart { index, part }, |id| Puzzle5dOperation::RemovePart { id }, Puzzle5dPart);
    collect_collection!("fasteners", |(index, fastener)| Puzzle5dOperation::SetFastener { index, fastener }, |id| Puzzle5dOperation::RemoveFastener { id }, Puzzle5dFastener);
    let before_meta = before_object.get("meta");
    let after_meta = after_object.get("meta");
    if before_meta != after_meta {
        let meta = match after_meta {
            Some(value) => match serde_json::from_value::<Puzzle5dMeta>(value.clone()) {
                Ok(meta) => meta,
                Err(_) => return fallback(after),
            },
            None => Puzzle5dMeta::default(),
        };
        operations.push(Puzzle5dOperation::SetMeta { meta });
    }
    let mut replay = before.clone();
    for operation in &operations {
        apply_puzzle5d_operation_to_value(&mut replay, operation);
    }
    if &replay == after {
        operations
    } else {
        fallback(after)
    }
}

// #region 🔖PlayProjection
/// 🌱 `puzzle-plugin`'s `Puzzle5dPlayApp` predates the typed `Puzzle5dProjection` above and stays on
/// this ad-hoc `serde_json::Value` fixture shape for its scene-mutation helpers (out of scope to
/// retrofit onto the typed struct). This newtype exists only to satisfy `DocumentApp::Projection:
/// store::DocumentDsl + store::DocumentPack` post the repo-wide `store::DocumentDsl for serde_json::Value`
/// bridge's removal (final DSL-syntax convergence gate); `parse_dsl`/`print_dsl`/`encode_pack_with`/
/// `decode_pack_with` all round-trip straight through the still-standing `serde_json::Value` impls
/// (JSON text / JSON-bridge pack encoding respectively), same local-bridge shape as `puzzle_2d`'s
/// `Puzzle2dPlayProjection`, `puzzle_3d`'s `Puzzle3dPlayProjection` and `compose`'s `KitSnapshot`.
/// `Operation`/`OperationDiff` delegate straight through to the `Value` impls above too.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Puzzle5dPlayProjection(pub Value);

impl store::DocumentDsl for Puzzle5dPlayProjection {
    const EXTENSION: &'static str = "puzzle5d-play";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map(Puzzle5dPlayProjection).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(&self.0).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle5dPlayProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.0.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Value::decode_pack_with(bytes, options).map(Puzzle5dPlayProjection)
    }
}

impl OperationDiff<Puzzle5dPlayProjection> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlayProjection) -> Puzzle5dPlayProjection {
        Puzzle5dPlayProjection(OperationDiff::<Value>::apply(self, &projection.0))
    }

    fn absorb(&mut self, other: Self) {
        puzzle5d_diff_absorb(self, other);
    }
}

impl Operation<Puzzle5dPlayProjection> for Puzzle5dOperation {
    type Diff = Puzzle5dDiff;

    fn diff(&self, projection: &Puzzle5dPlayProjection) -> Puzzle5dDiff {
        Operation::<Value>::diff(self, &projection.0)
    }

    fn backwards(&self, projection: &Puzzle5dPlayProjection) -> Vec<Self> {
        Operation::<Value>::backwards(self, &projection.0)
    }
}
// #endregion 🔖PlayProjection
// #endregion 🔖ValueBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_delta_ops_round_trip_and_stay_granular() {
        let before = serde_json::json!({
            "schema": puzzle_5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p1", "2d": { "x": 0.0, "y": 0.0 }, "3d": { "origin": [0.0,0.0,0.0] }, "grips": [] },
                { "id": "p2", "2d": { "x": 1.0, "y": 0.0 }, "3d": { "origin": [1.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let after = serde_json::json!({
            "schema": puzzle_5d::PUZZLE_5D_SCHEMA, "domain": "architecture",
            "meta": { "description": "" },
            "parts": [
                { "id": "p2", "2d": { "x": 9.0, "y": 0.0 }, "3d": { "origin": [9.0,0.0,0.0] }, "grips": [] },
                { "id": "p3", "2d": { "x": 2.0, "y": 0.0 }, "3d": { "origin": [2.0,0.0,0.0] }, "grips": [] },
            ],
            "fasteners": [],
        });
        let operations = puzzle5d_document_delta_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dOperation::SetPart { .. })));
        assert!(!operations.iter().any(|operation| matches!(operation, Puzzle5dOperation::SetDocument { .. })), "granular delta must not fall back to whole-document replace here");
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
