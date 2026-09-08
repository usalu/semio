
use super::*;
use crate::editor::gis2d::Gis2dCommand;
use crate::editor::gis2d::testkit::{app, dispatch};

const ROUTE_A: &str = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
const ROUTE_B: &str = "bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0";

#[semio_framework_async_macros::async_test]
async fn patch_routes_emits_route_patch_ops_and_updates_document() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "Renamed Route".into() })).await;
    assert_eq!(result.mutations.len(), 1, "one matching route → one patch operation");
    let document = app.snapshot().expect("projection");
    let route = document.routes.iter().find(|route| route.id == ROUTE_A).expect("route");
    assert_eq!(route.data.get("label").and_then(|value| value.as_str()), Some("Renamed Route"));
}

#[semio_framework_async_macros::async_test]
async fn patch_routes_with_no_ids_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: "x".into() })).await;
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_positions_with_malformed_json_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: "not json".into() })).await;
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_positions_diffs_the_incoming_array_into_granular_operations() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"patched-1","lon":1.0,"lat":2.0}]"#.into() })).await;
    let document = app.snapshot().expect("projection");
    assert!(document.positions.iter().any(|feature| feature.id == "patched-1"), "the incoming array is diffed into a granular add");
    assert_eq!(document.positions.len(), 1, "features absent from the incoming array are removed");
}

/// 🤝️ Definitional merge proof: two instances on one backbone patch DIFFERENT routes; after
/// exchanging operations both converge and keep both edits — impossible under whole-map LWW snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_route_edits() {
    let command_a = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "A".into() });
    let command_b = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_B.into(), field: "label".into(), value: "B".into() });
    let label = |document: &GisMapSnapshot, id: &str| document.routes.iter().find(|route| route.id == id).and_then(|route| route.data.get("label").and_then(|value| value.as_str().map(str::to_string)));
    semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<crate::editor::gis2d::Gis2dPlayApp>, _>("mem://gis2d-convergence", command_a, command_b, |app| {
        let document = app.snapshot().expect("projection");
        (label(&document, ROUTE_A), label(&document, ROUTE_B))
    })
    .await;
}
