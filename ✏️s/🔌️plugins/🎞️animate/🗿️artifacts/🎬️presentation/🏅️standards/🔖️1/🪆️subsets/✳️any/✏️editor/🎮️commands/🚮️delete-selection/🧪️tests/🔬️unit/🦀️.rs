use super::*;
use crate::editor::animate::unit_tests::context::{dispatch, presentation_app_with_registry};
use crate::editor::animate::{commands::add_tile, PresentationCommand, PRESENTATION_INTERACTION_DOMAIN, PRESENTATION_INTERACTION_GRANULARITY};
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{PluginApp, CLEAR_SELECTION_ACTION_ID, INTERACTION_SELECT_ACTION_ID};

/// 🕹️ End-to-end proof the `tiles` domain's live selection actually drives `deleteSelection` —
/// adds a tile, selects it via the framework's real `interactionSelect` action (the only way a
/// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
/// removes exactly that tile.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_tile() {
    let mut app = presentation_app_with_registry().await;
    dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
    let targets = dsl::os_pack::json::to_string(&dsl::os_pack::json::Value::Array(vec![dsl::os_pack::json::object([
        ("granularity".to_string(), dsl::os_pack::json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)),
        ("id".to_string(), dsl::os_pack::json::Value::from(tile_id.clone())),
    ])]));
    let args = dsl::DslValue::object([
        ("domainId".to_string(), dsl::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
        ("targets".to_string(), dsl::DslValue::String(targets)),
        ("merge".to_string(), dsl::DslValue::String("replace".into())),
        ("method".to_string(), dsl::DslValue::String("pick".into())),
    ]);
    let admitted = app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interactionSelect");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admitted).await.expect("interactionSelect settles");
    dispatch(&mut app, PresentationCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.is_empty(), "selected tile must be deleted");
}

/// 🫥️ `addTile` selects the tile it creates, so the empty selection is established explicitly through
/// the framework's reserved `clearSelection` route — admitted, then settled like the host settles it —
/// before deleting.
#[semio_framework_async_macros::async_test]
async fn delete_selection_with_no_selection_is_a_no_op() {
    let mut app = presentation_app_with_registry().await;
    dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
    let admitted = app.handle_action(CLEAR_SELECTION_ACTION_ID, None, &meta("local")).await.expect("clearSelection");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app, admitted).await.expect("clearSelection settles");
    dispatch(&mut app, PresentationCommand::DeleteSelection(DeleteSelection {})).await;
    assert_eq!(crate::presentation_working_scene(&app.snapshot().expect("projection")).1.len(), 1, "nothing selected means nothing deleted");
}
