//! ⚡ GIS 2D app — operation enum + laws (constitutional: op).

use gis2d::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Operation, OperationDiff, Patchable};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖Types
fn apply_map_collection_diff(items: &mut Vec<MapFeature>, diff: &CollectionDiff<String, MapFeaturePatch, MapFeature>) {
    for id in &diff.removed {
        items.retain(|item| &item.id != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id == patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

fn absorb_map_collection_diff(
    target: &mut Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
    incoming: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
) {
    if let Some(next) = incoming {
        match target {
            Some(existing) => {
                existing.removed.extend(next.removed);
                existing.modified.extend(next.modified);
                existing.added.extend(next.added);
            }
            None => *target = Some(next),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapDiff {
    pub document: Option<GisMapDocument>,
    pub positions: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
    pub routes: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
    pub regions: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
}

impl OperationDiff<GisMapDocument> for GisMapDiff {
    fn apply(&self, projection: &GisMapDocument) -> GisMapDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(diff) = &self.positions {
            apply_map_collection_diff(&mut next.positions, diff);
        }
        if let Some(diff) = &self.routes {
            apply_map_collection_diff(&mut next.routes, diff);
        }
        if let Some(diff) = &self.regions {
            apply_map_collection_diff(&mut next.regions, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = GisMapDiff { document: other.document, ..Default::default() };
            return;
        }
        absorb_map_collection_diff(&mut self.positions, other.positions);
        absorb_map_collection_diff(&mut self.routes, other.routes);
        absorb_map_collection_diff(&mut self.regions, other.regions);
    }
}

/// 🗺️ Typed, invertible map operation. `Positions`/`Routes`/`Regions` are id-keyed collection operations for
/// granular convergence; `SetDocument` replaces the whole map (example import / reset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum GisMapOperation {
    Positions(CollectionOperation<String, MapFeature, MapFeaturePatch>),
    Routes(CollectionOperation<String, MapFeature, MapFeaturePatch>),
    Regions(CollectionOperation<String, MapFeature, MapFeaturePatch>),
    SetDocument { document: GisMapDocument },
}

impl Operation<GisMapDocument> for GisMapOperation {
    type Diff = GisMapDiff;

    fn diff(&self, projection: &GisMapDocument) -> GisMapDiff {
        match self {
            GisMapOperation::Positions(operation) => GisMapDiff { positions: Some(collection_diff_from_operation(&projection.positions, operation)), ..Default::default() },
            GisMapOperation::Routes(operation) => GisMapDiff { routes: Some(collection_diff_from_operation(&projection.routes, operation)), ..Default::default() },
            GisMapOperation::Regions(operation) => GisMapDiff { regions: Some(collection_diff_from_operation(&projection.regions, operation)), ..Default::default() },
            GisMapOperation::SetDocument { document } => GisMapDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &GisMapDocument) -> Vec<Self> {
        match self {
            GisMapOperation::Positions(operation) => vec![GisMapOperation::Positions(invert_collection_operation(&projection.positions, operation))],
            GisMapOperation::Routes(operation) => vec![GisMapOperation::Routes(invert_collection_operation(&projection.routes, operation))],
            GisMapOperation::Regions(operation) => vec![GisMapOperation::Regions(invert_collection_operation(&projection.regions, operation))],
            GisMapOperation::SetDocument { .. } => vec![GisMapOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type GisMapEnvelope = DocumentEnvelope<GisMapDocument, GisMapOperation>;
pub type GisMapStore = DocumentStore<GisMapDocument, GisMapOperation>;
//#endregion 🔖Types

//#region 🔖OpText
/// ✂️ Local DSL-only mirror of `GisMapOperation` — `protocol::CollectionOperation<K,V,P>` is declared
/// in the `protocol` crate (foreign type), so it cannot itself gain a `dsl::DslField`/`dsl::DslVariants`
/// binding here (orphan rule: neither the trait nor the type is local to this crate). This twin
/// flattens each `Positions|Routes|Regions { collection }` wrapper into its own four keyworded
/// variants — mirroring `process::Process3dOperationDsl`'s identical fix for the same foreign-
/// `CollectionOperation` problem — and converts at the `protocol::OpText` boundary only; `GisMapOperation`
/// itself, and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum GisMapOperationDsl {
    AddPosition { index: usize, #[dsl(block)] item: MapFeature },
    RemovePosition { id: String },
    MovePosition { id: String, #[dsl(key = "to")] to_index: usize },
    PatchPosition { id: String, #[dsl(block)] patch: MapFeaturePatch },
    AddRoute { index: usize, #[dsl(block)] item: MapFeature },
    RemoveRoute { id: String },
    MoveRoute { id: String, #[dsl(key = "to")] to_index: usize },
    PatchRoute { id: String, #[dsl(block)] patch: MapFeaturePatch },
    AddRegion { index: usize, #[dsl(block)] item: MapFeature },
    RemoveRegion { id: String },
    MoveRegion { id: String, #[dsl(key = "to")] to_index: usize },
    PatchRegion { id: String, #[dsl(block)] patch: MapFeaturePatch },
    SetDocument { #[dsl(block)] document: GisMapDocument },
}

fn gis_map_operation_to_dsl(operation: &GisMapOperation) -> GisMapOperationDsl {
    match operation {
        GisMapOperation::Positions(CollectionOperation::Add { id: _id, item, at }) => GisMapOperationDsl::AddPosition { index: *at, item: item.clone() },
        GisMapOperation::Positions(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemovePosition { id: id.clone() },
        GisMapOperation::Positions(CollectionOperation::Move { id, to }) => GisMapOperationDsl::MovePosition { id: id.clone(), to_index: *to },
        GisMapOperation::Positions(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchPosition { id: id.clone(), patch: patch.clone() },
        GisMapOperation::Routes(CollectionOperation::Add { id: _id, item, at }) => GisMapOperationDsl::AddRoute { index: *at, item: item.clone() },
        GisMapOperation::Routes(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemoveRoute { id: id.clone() },
        GisMapOperation::Routes(CollectionOperation::Move { id, to }) => GisMapOperationDsl::MoveRoute { id: id.clone(), to_index: *to },
        GisMapOperation::Routes(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchRoute { id: id.clone(), patch: patch.clone() },
        GisMapOperation::Regions(CollectionOperation::Add { id: _id, item, at }) => GisMapOperationDsl::AddRegion { index: *at, item: item.clone() },
        GisMapOperation::Regions(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemoveRegion { id: id.clone() },
        GisMapOperation::Regions(CollectionOperation::Move { id, to }) => GisMapOperationDsl::MoveRegion { id: id.clone(), to_index: *to },
        GisMapOperation::Regions(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchRegion { id: id.clone(), patch: patch.clone() },
        GisMapOperation::SetDocument { document } => GisMapOperationDsl::SetDocument { document: document.clone() },
    }
}

fn gis_map_operation_from_dsl(operation: GisMapOperationDsl) -> GisMapOperation {
    match operation {
        GisMapOperationDsl::AddPosition { index, item } => GisMapOperation::Positions(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        GisMapOperationDsl::RemovePosition { id } => GisMapOperation::Positions(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MovePosition { id, to_index } => GisMapOperation::Positions(CollectionOperation::Move { id, to: to_index }),
        GisMapOperationDsl::PatchPosition { id, patch } => GisMapOperation::Positions(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::AddRoute { index, item } => GisMapOperation::Routes(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        GisMapOperationDsl::RemoveRoute { id } => GisMapOperation::Routes(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MoveRoute { id, to_index } => GisMapOperation::Routes(CollectionOperation::Move { id, to: to_index }),
        GisMapOperationDsl::PatchRoute { id, patch } => GisMapOperation::Routes(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::AddRegion { index, item } => GisMapOperation::Regions(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        GisMapOperationDsl::RemoveRegion { id } => GisMapOperation::Regions(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MoveRegion { id, to_index } => GisMapOperation::Regions(CollectionOperation::Move { id, to: to_index }),
        GisMapOperationDsl::PatchRegion { id, patch } => GisMapOperation::Regions(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::SetDocument { document } => GisMapOperation::SetDocument { document },
    }
}

impl protocol::OpText for GisMapOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(gis_map_operation_from_dsl(<GisMapOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <GisMapOperationDsl as protocol::OpText>::print_op(&gis_map_operation_to_dsl(self))
    }
}

/// ⚡ Binary mirror of the `OpText` impl above — `GisMapOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for GisMapOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        gis_map_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(gis_map_operation_from_dsl(GisMapOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖OpText

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use store::{create_document_envelope, DocumentCommand};

    fn round_trip(document: &GisMapDocument, operation: &GisMapOperation) -> GisMapDocument {
        let forward = vcs::apply_operation(document, operation);
        let backwards = operation.backwards(document);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, document, "backwards() must exactly restore the pre-operation document");
        forward
    }

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: json!({ "id": id, "lon": 1.0, "lat": 2.0 }) }
    }

    #[test]
    fn positions_add_patch_remove_round_trip() {
        let document = GisMapDocument::default();
        let added = round_trip(&document, &GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: feature("p1"), at: 0 }));
        assert_eq!(added.positions.len(), 1);
        let patched = round_trip(
            &added,
            &GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(json!({ "id": "p1", "label": "Home" })) } }),
        );
        assert_eq!(patched.positions[0].data.get("label").and_then(|value| value.as_str()), Some("Home"));
        let removed = round_trip(&patched, &GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() }));
        assert!(removed.positions.is_empty());
    }

    #[test]
    fn descriptor_round_trips_through_document() {
        let json = r#"{"positions":[{"id":"a","lon":1.0,"lat":2.0}],"routes":[{"id":"r","points":[]}],"regions":[]}"#;
        let document = gis2d_engine::gis_map_document_from_descriptor_json(json);
        assert_eq!(document.positions.len(), 1);
        assert_eq!(document.routes.len(), 1);
        let rebuilt = gis2d_engine::gis_map_document_from_descriptor_json(&gis2d_engine::gis_map_descriptor_json(&document));
        assert_eq!(rebuilt, document);
    }

    #[test]
    fn gis_map_document_vcs_replays_operations() {
        let mut store = GisMapStore::new(create_document_envelope(gis2d::GIS_MAP_SCHEMA, "gis", gis2d_engine::empty_gis_map_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: feature("p1"), at: 0 })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").positions.len(), 1);
    }

    fn sample_patch_feature() -> MapFeature {
        MapFeature { id: "p1".into(), data: json!({ "id": "p1", "lon": 1.0, "lat": 2.0 }) }
    }

    #[test]
    fn gis_map_positions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Move { id: "p1".into(), to: 3 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Patch {
            id: "p1".into(),
            patch: MapFeaturePatch { data: Some(json!({ "label": "Home" })) },
        }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: None } }));
    }

    #[test]
    fn gis_map_routes_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Move { id: "p1".into(), to: 1 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Patch {
            id: "p1".into(),
            patch: MapFeaturePatch { data: Some(json!({ "kind": "reuse" })) },
        }));
    }

    #[test]
    fn gis_map_regions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Move { id: "p1".into(), to: 2 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Patch {
            id: "p1".into(),
            patch: MapFeaturePatch { data: Some(json!({ "kind": "boundary" })) },
        }));
    }

    #[test]
    fn gis_map_set_document_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::SetDocument { document: gis2d_engine::default_document() });
    }
}
//#endregion 🧪Tests
