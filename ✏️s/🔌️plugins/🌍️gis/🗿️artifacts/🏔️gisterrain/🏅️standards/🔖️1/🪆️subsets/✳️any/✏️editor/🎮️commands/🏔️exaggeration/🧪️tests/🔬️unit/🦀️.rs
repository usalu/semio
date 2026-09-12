use super::*;
use crate::editor::gis3d::unit_tests::context::{app, close, dispatch};
use crate::editor::gis3d::Gis3dCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn seeds_exaggeration_from_the_terrain_fixture() {
    let mut app = app().await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5);
    close(&mut app);
}

/// 🧪️ A slider drag is many `setExaggeration` ticks sharing one coalesce key: they fold into ONE
/// undoable edit, so a single undo restores the fixture's exaggeration rather than a mid-drag value.
#[semio_framework_async_macros::async_test]
async fn exaggeration_drag_coalesces_into_one_undo_step() {
    let mut app = app().await;
    for value in [2.0, 2.5, 3.0] {
        dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: value })).await;
    }
    assert_eq!(app.snapshot().expect("projection").exaggeration, 3.0);
    app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5, "one coalesced edit: undo restores the fixture exaggeration");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn set_exaggeration_is_a_document_operation_not_config_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).await;
    assert_eq!(result.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Artifact).count(), 1, "exaggeration publishes exactly one undoable document operation");
    close(&mut app);
}
