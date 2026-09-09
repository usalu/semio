use super::*;
use crate::editor::animate::commands::{delete_selection, delete_tile, patch_tile_crops, rename_tiles};
use crate::editor::animate::testkit::{dispatch, presentation_app, presentation_app_with_registry};
use crate::editor::animate::PresentationCommand;
use semio_framework_plugin::testkit::meta;

async fn seed_2x2(app: &mut crate::editor::animate::testkit::PresentationApp) {
    dispatch(app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
}

#[semio_framework_async_macros::async_test]
async fn add_delete_and_rename_tile_round_trip_through_operations() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::AddTile(AddTile { crop: None }), &meta("local")).await.expect("add tile");
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
    app.dispatch_typed(PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id.clone()], value: "Hero".into() }), &meta("local")).await.expect("rename");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].name, "Hero");
    app.dispatch_typed(PresentationCommand::DeleteTile(delete_tile::DeleteTile { id: tile_id }), &meta("local")).await.expect("delete");
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_tile_crop_clamps_and_is_reversible() {
    use semio_framework_plugin::PluginApp;
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::AddTile(AddTile { crop: None }), &meta("local")).await.expect("add tile");
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
    app.dispatch_typed(PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec![tile_id], field: "width".into(), value: 0.5 }), &meta("local")).await.expect("patch crop");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].crop.width, 0.5);
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].crop.width, 0.2);
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); selects the live tile through the real `interactionSelect` action (the only way a
/// downstream crate can populate a genuine `InteractionView`, see
/// `🎮️commands/🚮️delete-selection`'s own tests for the equivalent single-tile case).
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_only_the_selected_tile() {
    use crate::editor::animate::{PRESENTATION_INTERACTION_DOMAIN, PRESENTATION_INTERACTION_GRANULARITY};
    use semio_framework_plugin::{PluginApp, INTERACTION_SELECT_ACTION_ID};
    let mut app = presentation_app_with_registry().await;
    seed_2x2(&mut app).await;
    let first_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
    let targets = dsl::os_pack::json::to_string(&dsl::os_pack::json::Value::Array(vec![dsl::os_pack::json::object([
        ("granularity".to_string(), dsl::os_pack::json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)),
        ("id".to_string(), dsl::os_pack::json::Value::from(first_id)),
    ])]));
    let args = dsl::DslValue::object([
        ("domainId".to_string(), dsl::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
        ("targets".to_string(), dsl::DslValue::String(targets)),
        ("merge".to_string(), dsl::DslValue::String("replace".into())),
        ("method".to_string(), dsl::DslValue::String("pick".into())),
    ]);
    app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("select");
    app.dispatch_typed(PresentationCommand::DeleteSelection(delete_selection::DeleteSelection {}), &meta("local")).await.expect("delete selection");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 3, "only the selected tile is removed");
}

#[semio_framework_async_macros::async_test]
async fn delete_tile_with_unknown_id_is_a_no_op() {
    let mut app = presentation_app().await;
    seed_2x2(&mut app).await;
    app.dispatch_typed(PresentationCommand::DeleteTile(delete_tile::DeleteTile { id: "does-not-exist".into() }), &meta("local")).await.expect("delete missing");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 4, "unknown ids are filtered out before dispatch");
}

#[semio_framework_async_macros::async_test]
async fn rename_tiles_with_blank_value_leaves_name_unchanged() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::AddTile(AddTile { crop: None }), &meta("local")).await.expect("add tile");
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
    let before = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].name.clone();
    app.dispatch_typed(PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id], value: "   ".into() }), &meta("local")).await.expect("rename blank");
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].name, before, "whitespace-only rename is rejected");
}

#[semio_framework_async_macros::async_test]
async fn rename_tiles_with_unknown_ids_is_a_no_op() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::AddTile(AddTile { crop: None }), &meta("local")).await.expect("add tile");
    app.dispatch_typed(PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec!["nope".into()], value: "Hero".into() }), &meta("local")).await.expect("rename unknown");
    assert_ne!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].name, "Hero");
}

#[semio_framework_async_macros::async_test]
async fn patch_tile_crops_covers_all_fields_across_multiple_tiles() {
    let mut app = presentation_app().await;
    seed_2x2(&mut app).await;
    let ids: Vec<String> = crate::presentation_working_scene(&app.snapshot().expect("projection")).1.iter().map(|tile| tile.id.clone()).collect();
    for field in ["x", "y", "width", "height"] {
        app.dispatch_typed(PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: ids.clone(), field: field.into(), value: 0.4 }), &meta("local")).await.expect("patch field");
    }
    for tile in &crate::presentation_working_scene(&app.snapshot().expect("projection")).1 {
        assert_eq!(tile.crop.width, 0.4);
        assert_eq!(tile.crop.height, 0.4);
    }
}

#[semio_framework_async_macros::async_test]
async fn patch_tile_crops_targeting_no_existing_tile_is_a_no_op() {
    let mut app = presentation_app().await;
    app.dispatch_typed(PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec!["ghost".into()], field: "width".into(), value: 0.4 }), &meta("local")).await.expect("patch ghost");
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn app_manifest_declares_expected_operations() {
    use semio_framework_plugin::ActionKind;
    let definition = crate::editor::animate::create_animate_presentation_app();
    let operation_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).filter(|action| matches!(action.kind, ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
    for expected in ["addTile", "deleteTile", "deleteSelection", "renameTiles", "patchTileCrops"] {
        assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
    }
    let _ = presentation_app_with_registry().await;
}
