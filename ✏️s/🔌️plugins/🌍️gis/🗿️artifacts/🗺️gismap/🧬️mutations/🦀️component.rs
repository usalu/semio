//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gismap::diff::{diff_set_snapshot, features_delta_from_collection_mutation, GisMapDiff};
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature, MapFeaturePatch};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible map operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GisMapMutation {
    Positions(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    Routes(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    Regions(CollectionMutation<String, MapFeature, MapFeaturePatch>),
    SetSnapshot { snapshot: GisMapSnapshot },
}

impl Mutation<GisMapSnapshot> for GisMapMutation {
    type Diff = GisMapDiff;

    fn diff(&self, snapshot: &GisMapSnapshot) -> GisMapDiff {
        match self {
            GisMapMutation::Positions(operation) => GisMapDiff {
                positions: Some(features_delta_from_collection_mutation(&snapshot.positions, operation)),
                ..Default::default()
            },
            GisMapMutation::Routes(operation) => GisMapDiff {
                routes: Some(features_delta_from_collection_mutation(&snapshot.routes, operation)),
                ..Default::default()
            },
            GisMapMutation::Regions(operation) => GisMapDiff {
                regions: Some(features_delta_from_collection_mutation(&snapshot.regions, operation)),
                ..Default::default()
            },
            GisMapMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &GisMapSnapshot) -> Vec<Self> {
        match self {
            GisMapMutation::Positions(operation) => vec![GisMapMutation::Positions(inverse_collection_mutation(&snapshot.positions, operation))],
            GisMapMutation::Routes(operation) => vec![GisMapMutation::Routes(inverse_collection_mutation(&snapshot.routes, operation))],
            GisMapMutation::Regions(operation) => vec![GisMapMutation::Regions(inverse_collection_mutation(&snapshot.regions, operation))],
            GisMapMutation::SetSnapshot { .. } => vec![GisMapMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

pub type GisMapEnvelope = DocumentEnvelope<GisMapSnapshot, GisMapMutation>;
pub type GisMapStore = DocumentStore<GisMapSnapshot, GisMapMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::engine::{empty_gis_map_snapshot, gis_map_descriptor_json, gis_map_document_from_descriptor_json};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;
    use store::{create_document_envelope, DocumentCommand};

    fn round_trip(document: &GisMapSnapshot, operation: &GisMapMutation) -> GisMapSnapshot {
        let forward = vcs::apply_mutation(document, operation);
        let backwards = operation.inverse(document);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_mutation(&restored, back);
        }
        assert_eq!(&restored, document, "inverse must exactly restore the pre-operation document");
        forward
    }

    fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
    }

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[test]
    fn positions_add_patch_remove_round_trip() {
        let document = GisMapSnapshot::default();
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
        let mut store = GisMapStore::new(create_document_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_snapshot(), None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: feature("p1") })], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").positions.len(), 1);
    }
}
//#endregion 🔹Tests

pub fn apply_gis_map_mutation(snapshot: &mut GisMapSnapshot, mutation: &GisMapMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_gis_map_mutation(snapshot: &GisMapSnapshot, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    mutation.inverse(snapshot)
}
