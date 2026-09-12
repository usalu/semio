use super::*;
use crate::schema::{empty_gis_map_snapshot, gis_map_descriptor_json, gis_map_document_from_descriptor_json};
use crate::GIS_MAP_SCHEMA;
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
    dsl::DslValue::from(value)
}

fn feature(id: &str) -> crate::MapFeature {
    crate::MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
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
    let document = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() });
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
    let base = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() });
    let mutation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 1, item: feature("p2") });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 2, item: feature("p3") }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_route_obeys_the_inverse_law() {
    let base = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1")], ..Default::default() });
    let mutation = GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id: "r1".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_region_data_obeys_the_inverse_and_diff_absorb_laws() {
    let base = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { regions: vec![feature("g1")], ..Default::default() });
    let mutation = GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "boundary" })) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "g1".into(), new_data: dsl_of(&json!({ "kind": "district" })) }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_routes_obeys_the_inverse_law() {
    let base = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1"), feature("r2")], ..Default::default() });
    let mutation = GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "r1".into(), to_index: 1 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
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
    let mut store = GisMapStore::new(create_document_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_snapshot(), None)).await.expect("map store");
    store.install_document_store_owners_exact(crate::spr::gis_map_document_store_owners());
    store.dispatch(ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("p1") })], description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").positions.len(), 1);
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<GisMapSnapshot, GisMapMutation>::new();
    for _ in 0..100_000 {
        if matches!(disposer.close_step(&mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("GIS document store close step"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&store));
    drop(disposer);
    drop(store);
}

//#region 🔖️OutcomeLaws
/// 🪧 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS Pass 3 — one test per
/// verb family, calling the test context laws landed under their frozen names
/// (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
/// `📡️spr/🧪️test/🦀️kit.rs`). `assert_outcome_policy_matrix` is not landed under that
/// name (only the differently-shaped `assert_policy_matrix`) — see this lane's report.
#[semio_framework_async_macros::async_test]
async fn delete_position_missing_target_is_error() {
    let base = GisMapSnapshot::default();
    let mutation = GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "gone".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_positions_missing_target_is_error() {
    let base = GisMapSnapshot::default();
    let mutation = GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "gone".into(), to_index: 0 });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_route_data_missing_target_is_error() {
    let base = GisMapSnapshot::default();
    let mutation = GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: "gone".into(), new_data: dsl::DslValue::Null });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_position_duplicate_id_fatal_never_applies() {
    let base = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() });
    let mutation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("p1") });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&Mutation::diff(&mutation, &base)).await;
}
//#endregion 🔖️OutcomeLaws
