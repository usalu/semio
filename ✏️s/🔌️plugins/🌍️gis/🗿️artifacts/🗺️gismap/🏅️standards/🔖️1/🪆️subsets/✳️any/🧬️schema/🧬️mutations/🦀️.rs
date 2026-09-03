//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::{
    create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data,
};
use crate::artifacts::gismap::GisMapSnapshot;
use dsl::{FromValue, ToValue};
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
#[derive(Clone, Debug, PartialEq, dsl::DslEnum, dsl::Mutations, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutations(snapshot = GisMapSnapshot, diff = GisMapDiff, schema = "gis.gismap")]
pub enum GisMapMutation {
    CreatePosition(create_position::CreatePosition),
    DeletePosition(delete_position::DeletePosition),
    ReplacePositionData(replace_position_data::ReplacePositionData),
    ReorderPositions(reorder_positions::ReorderPositions),
    CreateRoute(create_route::CreateRoute),
    DeleteRoute(delete_route::DeleteRoute),
    ReplaceRouteData(replace_route_data::ReplaceRouteData),
    ReorderRoutes(reorder_routes::ReorderRoutes),
    CreateRegion(create_region::CreateRegion),
    DeleteRegion(delete_region::DeleteRegion),
    ReplaceRegionData(replace_region_data::ReplaceRegionData),
    ReorderRegions(reorder_regions::ReorderRegions),
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
        let (forward, _messages) = vcs::apply_mutation(document, operation).expect("valid mutation");
        let backwards = operation.inverse(document);
        let mut restored = forward.clone();
        for back in &backwards {
            let (next, _messages) = vcs::apply_mutation(&restored, back).expect("valid inverse mutation");
            restored = next;
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

    #[semio_framework_async_macros::async_test]
    async fn positions_create_replace_delete_round_trip() {
        let document = GisMapSnapshot::default();
        let added = round_trip(&document, &GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("p1") }));
        assert_eq!(added.positions.len(), 1);
        let replaced = round_trip(&added, &GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "p1".into(), new_data: dsl_of(&json!({ "id": "p1", "label": "Home" })) }));
        assert_eq!(replaced.positions[0].data.get("label").and_then(|value| value.as_str()), Some("Home"));
        let removed = round_trip(&replaced, &GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "p1".into() }));
        assert!(removed.positions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn positions_reorder_round_trips() {
        let document = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() });
        let reordered = round_trip(&document, &GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "p1".into(), to_index: 2 }));
        assert_eq!(reordered.positions.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), vec!["p2", "p3", "p1"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_and_replace_of_a_missing_id_invert_to_nothing() {
        let document = GisMapSnapshot::default();
        assert!(GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "gone".into() }).inverse(&document).is_empty());
        assert!(GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "gone".into(), new_data: dsl::DslValue::Null }).inverse(&document).is_empty());
        assert!(GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "gone".into(), to_index: 0 }).inverse(&document).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn create_position_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() });
        let mutation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 1, item: feature("p2") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 2, item: feature("p3") }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_route_obeys_the_inverse_law() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1")], ..Default::default() });
        let mutation = GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id: "r1".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_region_data_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { regions: vec![feature("g1")], ..Default::default() });
        let mutation = GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "boundary" })) });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "district" })) }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_routes_obeys_the_inverse_law() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1"), feature("r2")], ..Default::default() });
        let mutation = GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "r1".into(), to_index: 1 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_round_trips_through_document() {
        let json = r#"{"positions":[{"id":"a","lon":1.0,"lat":2.0}],"routes":[{"id":"r","points":[]}],"regions":[]}"#;
        let document = gis_map_document_from_descriptor_json(json);
        assert_eq!(document.positions.len(), 1);
        assert_eq!(document.routes.len(), 1);
        let rebuilt = gis_map_document_from_descriptor_json(&gis_map_descriptor_json(&document));
        assert_eq!(rebuilt, document);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_document_vcs_replays_operations() {
        let mut store = GisMapStore::new(create_document_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_snapshot(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("p1") })], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").positions.len(), 1);
    }

    //#region 🔖️OutcomeLaws
    /// 🪧 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS Pass 3 — one test per
    /// verb family, calling the testkit laws landed under their frozen names
    /// (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
    /// `📡️spr/🧪️test/🦀️kit.rs`). `assert_outcome_policy_matrix` is not landed under that
    /// name (only the differently-shaped `assert_policy_matrix`) — see this lane's report.
    #[semio_framework_async_macros::async_test]
    async fn delete_position_missing_target_is_error() {
        let base = GisMapSnapshot::default();
        let mutation = GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "gone".into() });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_positions_missing_target_is_error() {
        let base = GisMapSnapshot::default();
        let mutation = GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "gone".into(), to_index: 0 });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_route_data_missing_target_is_error() {
        let base = GisMapSnapshot::default();
        let mutation = GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: "gone".into(), new_data: dsl::DslValue::Null });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_position_duplicate_id_fatal_never_applies() {
        let base = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() });
        let mutation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("p1") });
        protocol::testkit::assert_fatal_never_applies(&Mutation::diff(&mutation, &base));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🔹Tests

pub fn apply_gis_map_mutation(snapshot: &mut GisMapSnapshot, mutation: &GisMapMutation) -> protocol::MutationApplyResult<()> {
    let (next, _messages) = vcs::apply_mutation(snapshot, mutation)?;
    // 🕸️ `drawing`/`value` are pure functions of `(positions, routes, regions)` — re-derive them
    // after every mutation so the composed children never drift from what they actually describe
    // (see `crate::artifacts::gismap::🦀️.rs`'s `🔖️Composition` region doc).
    *snapshot = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(next);
    Ok(())
}

pub fn inverse_gis_map_mutation(snapshot: &GisMapSnapshot, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    mutation.inverse(snapshot)
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `GisMapMutation` variant, in declaration order — the vocabulary the `gismap-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-gismap-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-position",
    "delete-position",
    "replace-position-data",
    "reorder-positions",
    "create-route",
    "delete-route",
    "replace-route-data",
    "reorder-routes",
    "create-region",
    "delete-region",
    "replace-region-data",
    "reorder-regions",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `GisMapMutation` nor
/// `GisMapSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
/// Every committed snapshot is funnelled through `gis_map_snapshot_with_derived_children` on the way
/// in — `std`'s `DefaultHasher` leaves its digest unspecified, so the two derived child handles are
/// committed as readable placeholders rather than frozen values, and this is the same call the
/// subset's own fixture tests make. Funnelling BOTH the base and the expected after-snapshot keeps the
/// adapter's comparison exact instead of exempting a field.
///
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn gis_map_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<GisMapSnapshot, String> {
        let decoded: GisMapSnapshot = dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(crate::artifacts::gismap::gis_map_snapshot_with_derived_children(decoded))
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisMapMutation = dsl::os_pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <GisMapMutation as Mutation<GisMapSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::os_pack::json::object([
        ("base".to_string(), dsl::os_pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), dsl::os_pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), dsl::os_pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), dsl::os_pack::json::from_dsl_value(&forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), dsl::os_pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), dsl::os_pack::json::from_dsl_value(&inverse_messages.to_value())),
    ]);
    Ok(dsl::os_pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
