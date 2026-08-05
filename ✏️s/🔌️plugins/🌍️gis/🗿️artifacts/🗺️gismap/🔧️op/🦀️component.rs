//! ⚡️ GIS map artifact — the operation enum, its `Operation` law and the store aliases
//! (constitutional: op).

use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionOperation, Operation};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
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
//#endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::engine::{empty_gis_map_projection, gis_map_descriptor_json, gis_map_document_from_descriptor_json};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
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

    /// 🧬️ `MapFeature::data`/`MapFeaturePatch::data` are `dsl::DslValue` (see `crate::artifacts::gismap`'s
    /// doc comment) — this bridges a `serde_json::json!` literal into one for test-fixture ergonomics.
    fn dsl_of(value: serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(&value).unwrap_or(dsl::DslValue::Null)
    }

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: dsl_of(json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[test]
    fn positions_add_patch_remove_round_trip() {
        let document = GisMapDocument::default();
        let added = round_trip(&document, &GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: feature("p1"), at: 0 }));
        assert_eq!(added.positions.len(), 1);
        let patched = round_trip(&added, &GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(json!({ "id": "p1", "label": "Home" }))) } }));
        assert_eq!(patched.positions[0].data.get("label").and_then(|value| value.as_str()), Some("Home"));
        let removed = round_trip(&patched, &GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() }));
        assert!(removed.positions.is_empty());
    }

    #[test]
    fn descriptor_round_trips_through_document() {
        let json = r#"{"positions":[{"id":"a","lon":1.0,"lat":2.0}],"routes":[{"id":"r","points":[]}],"regions":[]}"#;
        let document = gis_map_document_from_descriptor_json(json);
        assert_eq!(document.positions.len(), 1);
        assert_eq!(document.routes.len(), 1);
        let rebuilt = gis_map_document_from_descriptor_json(&gis_map_descriptor_json(&document));
        assert_eq!(rebuilt, document);
    }

    #[test]
    fn gis_map_document_vcs_replays_operations() {
        let mut store = GisMapStore::new(create_document_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: feature("p1"), at: 0 })], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").positions.len(), 1);
    }
}
//#endregion 🧪️Tests
