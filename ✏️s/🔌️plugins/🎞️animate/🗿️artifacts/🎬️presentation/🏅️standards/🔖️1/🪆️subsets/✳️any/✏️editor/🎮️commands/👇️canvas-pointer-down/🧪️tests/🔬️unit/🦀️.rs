use super::*;
use crate::editor::animate::unit_tests::context::{dispatch, presentation_app_with_registry};
use crate::editor::animate::{commands::add_tile, PresentationCommand};
use semio_framework_plugin::Effect;

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_emits_interaction_select_for_a_hit_and_clears_on_miss() {
    let mut app = presentation_app_with_registry().await;
    dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();

    let hit = dispatch(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some(tile_id) })).await;
    assert!(matches!(hit.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));

    let miss = dispatch(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some("source-frame".into()) })).await;
    assert!(matches!(miss.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));
}
