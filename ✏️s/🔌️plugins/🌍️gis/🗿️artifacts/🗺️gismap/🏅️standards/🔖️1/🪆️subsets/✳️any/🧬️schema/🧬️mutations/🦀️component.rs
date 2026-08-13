//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::{
    create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data,
    replace_route_data,
};
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible, semantic GIS map mutation vocabulary — every variant wraps exactly one
/// `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `positions`/`routes`/`regions` are id-keyed `MapFeature` collections, each getting the same
/// four-verb vocabulary (`create`/`delete`/`replace-<noun>-data`/`reorder-<plural>`) per
/// `derivation-rules.md`'s per-id-keyed-collection recipe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[mutations(snapshot = GisMapSnapshot, diff = GisMapDiff, schema = "gis.gismap")]
pub enum GisMapMutation {
    CreatePosition(create_position::mutation::CreatePosition),
    DeletePosition(delete_position::mutation::DeletePosition),
    ReplacePositionData(replace_position_data::mutation::ReplacePositionData),
    ReorderPositions(reorder_positions::mutation::ReorderPositions),
    CreateRoute(create_route::mutation::CreateRoute),
    DeleteRoute(delete_route::mutation::DeleteRoute),
    ReplaceRouteData(replace_route_data::mutation::ReplaceRouteData),
    ReorderRoutes(reorder_routes::mutation::ReorderRoutes),
    CreateRegion(create_region::mutation::CreateRegion),
    DeleteRegion(delete_region::mutation::DeleteRegion),
    ReplaceRegionData(replace_region_data::mutation::ReplaceRegionData),
    ReorderRegions(reorder_regions::mutation::ReorderRegions),
}

pub type GisMapEnvelope = ArtifactEnvelope<GisMapSnapshot, GisMapMutation>;
pub type GisMapStore = ArtifactStore<GisMapSnapshot, GisMapMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::schema::{empty_gis_map_snapshot, gis_map_descriptor_json, gis_map_document_from_descriptor_json};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;
    use store::{create_document_envelope, ArtifactCommand};

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

    fn feature(id: &str) -> crate::artifacts::gismap::MapFeature {
        crate::artifacts::gismap::MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[test]
    fn positions_create_replace_delete_round_trip() {
        let document = GisMapSnapshot::default();
        let added = round_trip(&document, &GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 0, item: feature("p1") }));
        assert_eq!(added.positions.len(), 1);
        let replaced = round_trip(
            &added,
            &GisMapMutation::ReplacePositionData(replace_position_data::mutation::ReplacePositionData { id: "p1".into(), new_data: dsl_of(&json!({ "id": "p1", "label": "Home" })) }),
        );
        assert_eq!(replaced.positions[0].data.get("label").and_then(|value| value.as_str()), Some("Home"));
        let removed = round_trip(&replaced, &GisMapMutation::DeletePosition(delete_position::mutation::DeletePosition { id: "p1".into() }));
        assert!(removed.positions.is_empty());
    }

    #[test]
    fn positions_reorder_round_trips() {
        let document = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() });
        let reordered = round_trip(&document, &GisMapMutation::ReorderPositions(reorder_positions::mutation::ReorderPositions { id: "p1".into(), to_index: 2 }));
        assert_eq!(reordered.positions.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), vec!["p2", "p3", "p1"]);
    }

    #[test]
    fn delete_and_replace_of_a_missing_id_invert_to_nothing() {
        let document = GisMapSnapshot::default();
        assert!(GisMapMutation::DeletePosition(delete_position::mutation::DeletePosition { id: "gone".into() }).inverse(&document).is_empty());
        assert!(GisMapMutation::ReplacePositionData(replace_position_data::mutation::ReplacePositionData { id: "gone".into(), new_data: dsl::DslValue::Null }).inverse(&document).is_empty());
        assert!(GisMapMutation::ReorderPositions(reorder_positions::mutation::ReorderPositions { id: "gone".into(), to_index: 0 }).inverse(&document).is_empty());
    }

    #[test]
    fn create_position_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() });
        let mutation = GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 1, item: feature("p2") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 2, item: feature("p3") }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn delete_route_obeys_the_inverse_law() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1")], ..Default::default() });
        let mutation = GisMapMutation::DeleteRoute(delete_route::mutation::DeleteRoute { id: "r1".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn replace_region_data_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { regions: vec![feature("g1")], ..Default::default() });
        let mutation = GisMapMutation::ReplaceRegionData(replace_region_data::mutation::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "boundary" })) });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = GisMapMutation::ReplaceRegionData(replace_region_data::mutation::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "district" })) }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn reorder_routes_obeys_the_inverse_law() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1"), feature("r2")], ..Default::default() });
        let mutation = GisMapMutation::ReorderRoutes(reorder_routes::mutation::ReorderRoutes { id: "r1".into(), to_index: 1 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
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
        store
            .dispatch(ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index: 0, item: feature("p1") })], description: None })
            .expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").positions.len(), 1);
    }
}
//#endregion 🔹Tests

pub fn apply_gis_map_mutation(snapshot: &mut GisMapSnapshot, mutation: &GisMapMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
    // 🕸️ `drawing`/`value` are pure functions of `(positions, routes, regions)` — re-derive them
    // after every mutation so the composed children never drift from what they actually describe
    // (see `crate::artifacts::gismap::🦀️component.rs`'s `🔖️Composition` region doc).
    *snapshot = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(std::mem::take(snapshot));
}

pub fn inverse_gis_map_mutation(snapshot: &GisMapSnapshot, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    mutation.inverse(snapshot)
}
