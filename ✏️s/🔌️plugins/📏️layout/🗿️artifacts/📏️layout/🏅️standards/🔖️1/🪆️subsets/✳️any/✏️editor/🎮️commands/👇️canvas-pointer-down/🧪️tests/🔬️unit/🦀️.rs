
use super::*;
use crate::editor::layout::commands::{canvas_drag_leave, canvas_drag_over, canvas_drop, canvas_pointer_move, set_camera};
use crate::editor::layout::testkit::{dispatch, layout_app, render, test_screen_point};
use crate::editor::layout::{LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_SURFACE_PREVIEW, LayoutCommand};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID};

#[semio_framework_async_macros::async_test]
async fn set_camera_mutates_config_and_emits_no_operations() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection");
    let result = dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } })).await;
    assert!(result.mutations.is_empty(), "camera is a config action and emits no operations");
    assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_preview_surface_updates_independently_of_blueprint() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_PREVIEW.into()), camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 } })).await;
    let preview_json = render(&mut app, crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_BODY_PREVIEW).await;
    assert!(preview_json.contains(r#""cameraX":3.0"#), "preview scene reflects config camera: {preview_json}");
    let blueprint_json = render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await;
    assert!(blueprint_json.contains(r#""cameraX":0.0"#), "blueprint surface camera stays independent: {blueprint_json}");
}

/// 🕹️ Selection is framework-owned now: a hit no longer mutates config synchronously, it asks the
/// host to redispatch `interactionSelect` (`dispatch_interaction_action` runs that on the SAME
/// instance next, out of band — the test harness doesn't simulate the round trip, so this only
/// asserts the requested effect is shaped correctly, not that selection state landed).
#[semio_framework_async_macros::async_test]
async fn pointer_down_requests_a_select_effect_for_the_hit_frame() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(result.mutations.is_empty(), "pointer down never mutates the document directly");
    let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
    let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
    let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
    assert_eq!(args["domainId"], "elements");
    assert_eq!(args["merge"], "replace");
    assert!(args["targets"].as_str().expect("targets json").contains("frame-image-1"));
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_extend_click_requests_an_invertive_merge() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: true, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
    let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
    let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
    assert_eq!(args["merge"], "invertive");
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_on_empty_space_requests_clear_selection() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 5.0, 5.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == CLEAR_SELECTION_ACTION_ID)));
}

#[semio_framework_async_macros::async_test]
async fn pointer_move_requests_a_hover_effect_for_the_hit_frame() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert!(result.mutations.is_empty(), "hover never mutates the document directly");
    let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_HOVER_ACTION_ID)).expect("interactionHover effect");
    let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
    let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("hover args");
    assert!(args["targets"].as_str().expect("targets json").contains("frame-text-1"));
}

#[semio_framework_async_macros::async_test]
async fn canvas_drop_adds_frame_at_world_coords() {
    let mut app = layout_app().await;
    let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 100.0, 200.0);
    let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: sx, y: sy, width: 800.0, height: 600.0 })).await;
    assert_eq!(result.mutations.len(), 1);
    let doc = app.snapshot().expect("projection");
    let frame = doc.pages[0].frames.last().unwrap();
    let bounds = frame.bounds();
    assert!((bounds.x - 100.0).abs() < 0.01);
    assert!((bounds.y - 200.0).abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn canvas_drop_page_kind_adds_page() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection").pages.len();
    let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "page".into(), x: 0.0, y: 0.0, width: 800.0, height: 600.0 })).await;
    assert_eq!(result.mutations.len(), 1);
    assert_eq!(app.snapshot().expect("projection").pages.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn drag_over_emits_ghost_and_leave_clears() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 })).await;
    assert!(render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await.contains("layout.drop-preview"));

    dispatch(&mut app, LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {})).await;
    assert!(!render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).await.contains("layout.drop-preview"));
}
