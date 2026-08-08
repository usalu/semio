//! ⚡️ GIS map artifact — the mutation enum, its `Mutation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
/// 🗺️ Typed, invertible map operation. `Positions`/`Routes`/`Regions` are id-keyed collection operations for
/// granular convergence; `SetDocument` replaces the whole map (example import / reset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GisMapMutation {
    Positions(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    Routes(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    Regions(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    SetDocument { document: GisMapDocument },
}

impl Mutation<GisMapDocument> for GisMapMutation {
    type Diff = GisMapDiff;

    fn diff(&self, projection: &GisMapDocument) -> GisMapDiff {
        match self {
            GisMapMutation::Positions(operation) => GisMapDiff { positions: Some(collection_diff_from_mutation(&projection.positions, operation)), ..Default::default() },
            GisMapMutation::Routes(operation) => GisMapDiff { routes: Some(collection_diff_from_mutation(&projection.routes, operation)), ..Default::default() },
            GisMapMutation::Regions(operation) => GisMapDiff { regions: Some(collection_diff_from_mutation(&projection.regions, operation)), ..Default::default() },
            GisMapMutation::SetDocument { document } => GisMapDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &GisMapDocument) -> Vec<Self> {
        match self {
            GisMapMutation::Positions(operation) => vec![GisMapMutation::Positions(inverse_collection_mutation(&projection.positions, operation))],
            GisMapMutation::Routes(operation) => vec![GisMapMutation::Routes(inverse_collection_mutation(&projection.routes, operation))],
            GisMapMutation::Regions(operation) => vec![GisMapMutation::Regions(inverse_collection_mutation(&projection.regions, operation))],
            GisMapMutation::SetDocument { .. } => vec![GisMapMutation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type GisMapEnvelope = DocumentEnvelope<GisMapDocument, GisMapMutation>;
pub type GisMapStore = DocumentStore<GisMapDocument, GisMapMutation>;
//#endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::engine::{empty_gis_map_projection, gis_map_descriptor_json, gis_map_document_from_descriptor_json};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;
    use store::{create_document_envelope, DocumentCommand};

    fn round_trip(document: &GisMapDocument, operation: &GisMapMutation) -> GisMapDocument {
        let forward = vcs::apply_mutation(document, operation);
        let backwards = operation.inverse(document);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_mutation(&restored, back);
        }
        assert_eq!(&restored, document, "backwards() must exactly restore the pre-operation document");
        forward
    }

    /// 🧬️ `MapFeature::data`/`MapFeaturePatch::data` are `dsl::DslValue` (see `crate::artifacts::gismap`'s
    /// doc comment) — this bridges a `serde_json::json!` literal into one for test-fixture ergonomics.
    fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
    }

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[test]
    fn positions_add_patch_remove_round_trip() {
        let document = GisMapDocument::default();
        let added = round_trip(&document, &GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: feature("p1") }));
        assert_eq!(added.positions.len(), 1);
        let patched = round_trip(&added, &GisMapMutation::Positions(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "id": "p1", "label": "Home" }))) } }));
        assert_eq!(patched.positions[0].data.get("label").and_then(|value| value.as_str()), Some("Home"));
        let removed = round_trip(&patched, &GisMapMutation::Positions(CollectionMutation::Remove { id: "p1".into() }));
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
        store.dispatch(DocumentCommand::Apply { mutations: vec![GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: feature("p1") })], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").positions.len(), 1);
    }
}
//#endregion 🧪️Tests


pub fn apply_gis_map_mutation(projection: &mut GisMapDocument, mutation: &GisMapMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_gis_map_mutation(projection: &GisMapDocument, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    mutation.inverse(projection)
}
