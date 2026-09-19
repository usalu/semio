use super::*;
use crate::editor::gis2d::unit_tests::context::{app, assert_two_instances_converge, close, dispatch};
use crate::editor::gis2d::Gis2dCommand;

const ROUTE_A: &str = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
const ROUTE_B: &str = "bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0";

#[semio_framework_async_macros::async_test]
async fn patch_routes_emits_route_patch_ops_and_updates_document() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "Renamed Route".into() })).await;
    assert_eq!(result.artifact_publication_count(), 1, "one matching route publishes one patch operation");
    let document = app.snapshot().expect("projection");
    let route = document.routes.iter().find(|route| route.id == ROUTE_A).expect("route");
    assert_eq!(route.data.get("label").and_then(|value| value.as_str()), Some("Renamed Route"));
    drop(document);
    drop(result);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn patch_routes_with_no_ids_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: "x".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    drop(result);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn patch_positions_with_malformed_json_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: "not json".into() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    drop(result);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn patch_positions_diffs_the_incoming_array_into_granular_operations() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"patched-1","lon":1.0,"lat":2.0}]"#.into() })).await;
    let document = app.snapshot().expect("projection");
    assert!(document.positions.iter().any(|feature| feature.id == "patched-1"), "the incoming array is diffed into a granular add");
    assert_eq!(document.positions.len(), 1, "features absent from the incoming array are removed");
    drop(document);
    close(&mut app);
}

/// 🤝️ Definitional merge proof: two instances on one backbone patch DIFFERENT routes; after
/// exchanging operations both converge and keep both edits — impossible under whole-map LWW snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_route_edits() {
    let command_a = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "A".into() });
    let command_b = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_B.into(), field: "label".into(), value: "B".into() });
    let label = |document: &GisMapSnapshot, id: &str| document.routes.iter().find(|route| route.id == id).and_then(|route| route.data.get("label").and_then(|value| value.as_str().map(str::to_string)));
    assert_two_instances_converge("mem://gis2d-convergence", command_a, command_b, |app| {
        let document = app.snapshot().expect("projection");
        (label(&document, ROUTE_A), label(&document, ROUTE_B))
    })
    .await;
}

//#region 🗺️FeatureEditingVerbs
/// 🔎️ `(count, newest id)` of one collection — the probe the undo/redo round trips compare.
fn collection_witness(document: &GisMapSnapshot, collection: &str) -> (usize, Option<String>) {
    let features = match collection {
        "routes" => &document.routes,
        "regions" => &document.regions,
        _ => &document.positions,
    };
    (features.len(), features.last().map(|feature| feature.id.clone()))
}

fn number_at(value: &DslValue, index: usize) -> f64 {
    value.as_array().and_then(|pair| pair.get(index)).and_then(DslValue::as_f64).expect("numeric vertex component")
}

#[semio_framework_async_macros::async_test]
async fn add_feature_appends_a_position_and_undo_redo_round_trips() {
    let mut app = app().await;
    let before = collection_witness(&app.snapshot().expect("projection"), "positions");
    let after = (before.0 + 1, Some("position-1".to_string()));
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(
        &mut app,
        Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "positions".into(), label: "Depot".into(), lon: 7.5, lat: 51.0, span: 0.01 }),
        |app| collection_witness(&app.snapshot().expect("projection"), "positions"),
        before,
        after,
    )
    .await;
    let document = app.snapshot().expect("projection");
    let minted = document.positions.last().expect("minted position");
    assert_eq!(minted.data.get("label").and_then(|value| value.as_str()), Some("Depot"), "the staged label reaches the document");
    assert_eq!(minted.data.get("lon").and_then(|value| value.as_f64()), Some(7.5));
    assert_eq!(minted.data.get("lat").and_then(|value| value.as_f64()), Some(51.0));
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn add_feature_mints_drawable_geometry_for_routes_and_regions() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "routes".into(), label: String::new(), lon: 1.0, lat: 2.0, span: 0.5 })).await;
    dispatch(&mut app, Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "regions".into(), label: String::new(), lon: 1.0, lat: 2.0, span: 0.5 })).await;
    let document = app.snapshot().expect("projection");
    let route = document.routes.last().expect("minted route");
    assert_eq!(route.id, "route-1", "an unstaged label falls back to the minted id");
    let route_points = route.data.get("points").expect("route polyline");
    assert_eq!(route_points.as_array().map(<[DslValue]>::len), Some(2), "a route is a real two-vertex segment, never a degenerate point");
    let region_points = document.regions.last().expect("minted region").data.get("points").expect("region ring");
    assert_eq!(region_points.as_array().map(<[DslValue]>::len), Some(4), "a region is a closed quad");
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn move_feature_translates_a_route_polyline_whole() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let original = document.routes.iter().find(|route| route.id == ROUTE_A).expect("route").data.get("points").expect("polyline").clone();
    let vertices = original.as_array().expect("points");
    let span = (number_at(&vertices[1], 0) - number_at(&vertices[0], 0), number_at(&vertices[1], 1) - number_at(&vertices[0], 1));
    drop(document);
    dispatch(&mut app, Gis2dCommand::MoveFeature(move_feature::MoveFeature { collection: "routes".into(), feature_id: ROUTE_A.into(), lon: 0.0, lat: 0.0 })).await;
    let document = app.snapshot().expect("projection");
    let moved = document.routes.iter().find(|route| route.id == ROUTE_A).expect("route").data.get("points").expect("polyline").clone();
    let points = moved.as_array().expect("points");
    assert!(number_at(&points[0], 0).abs() < 1e-9 && number_at(&points[0], 1).abs() < 1e-9, "the anchor lands exactly on the staged coordinate");
    assert!((number_at(&points[1], 0) - span.0).abs() < 1e-9 && (number_at(&points[1], 1) - span.1).abs() < 1e-9, "the polyline keeps its shape — a move translates, it does not collapse");
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn rename_feature_retitles_the_addressed_position_and_leaves_geometry_alone() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let target = document.positions.first().expect("position").id.clone();
    let lon = document.positions.first().expect("position").data.get("lon").and_then(|value| value.as_f64());
    drop(document);
    dispatch(&mut app, Gis2dCommand::RenameFeature(rename_feature::RenameFeature { collection: "positions".into(), feature_id: target.clone(), label: "Harbour".into() })).await;
    let document = app.snapshot().expect("projection");
    let feature = document.positions.iter().find(|feature| feature.id == target).expect("position");
    assert_eq!(feature.data.get("label").and_then(|value| value.as_str()), Some("Harbour"));
    assert_eq!(feature.data.get("name").and_then(|value| value.as_str()), Some("Harbour"), "a payload that carries `name` keeps it in step with `label`");
    assert_eq!(feature.data.get("lon").and_then(|value| value.as_f64()), lon, "renaming never touches geometry");
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn rename_feature_with_an_empty_label_emits_nothing() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::RenameFeature(rename_feature::RenameFeature { collection: "positions".into(), feature_id: String::new(), label: String::new() })).await;
    assert_eq!(result.artifact_publication_count(), 0);
    drop(result);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn delete_feature_addresses_the_newest_entry_when_no_id_is_staged() {
    let mut app = app().await;
    dispatch(&mut app, Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "positions".into(), label: "Scratch".into(), lon: 0.0, lat: 0.0, span: 0.01 })).await;
    let document = app.snapshot().expect("projection");
    let after_add = collection_witness(&document, "positions");
    assert_eq!(after_add.1.as_deref(), Some("position-1"));
    drop(document);
    dispatch(&mut app, Gis2dCommand::DeleteFeature(delete_feature::DeleteFeature { collection: "positions".into(), feature_id: String::new() })).await;
    let document = app.snapshot().expect("projection");
    assert_eq!(collection_witness(&document, "positions").0, after_add.0 - 1, "the unstaged address removes exactly the newest entry");
    assert!(!document.positions.iter().any(|feature| feature.id == "position-1"));
    drop(document);
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn delete_feature_undo_redo_round_trips() {
    let mut app = app().await;
    let document = app.snapshot().expect("projection");
    let target = document.positions.first().expect("position").id.clone();
    let before = document.positions.len();
    drop(document);
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(
        &mut app,
        Gis2dCommand::DeleteFeature(delete_feature::DeleteFeature { collection: "positions".into(), feature_id: target }),
        |app| app.snapshot().expect("projection").positions.len(),
        before,
        before - 1,
    )
    .await;
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_collection_is_a_no_op_rather_than_a_fault() {
    let mut app = app().await;
    for command in [
        Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "layers".into(), label: String::new(), lon: 0.0, lat: 0.0, span: 0.01 }),
        Gis2dCommand::MoveFeature(move_feature::MoveFeature { collection: "layers".into(), feature_id: String::new(), lon: 0.0, lat: 0.0 }),
        Gis2dCommand::DeleteFeature(delete_feature::DeleteFeature { collection: "layers".into(), feature_id: String::new() }),
    ] {
        let result = dispatch(&mut app, command).await;
        assert_eq!(result.artifact_publication_count(), 0);
        drop(result);
    }
    close(&mut app);
}
//#endregion 🗺️FeatureEditingVerbs
