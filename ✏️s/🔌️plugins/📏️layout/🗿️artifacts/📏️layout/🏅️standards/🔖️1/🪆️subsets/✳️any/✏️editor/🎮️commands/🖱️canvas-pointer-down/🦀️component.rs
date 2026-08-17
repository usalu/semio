//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-down`.

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::commands::{add_frame, add_page};
use crate::editor::layout::config::LayoutConfig;
use crate::artifacts::layout::LayoutDropPreviewState;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutCamera, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `dispatch_action` intercepts the six
// framework interaction verbs BEFORE routing to `ArtifactApp::handle`, so `LayoutCommand::dispatch`
// can no longer emit a selection mutation of its own; the hit test below is unchanged, only its
// result now travels as a `crate::editor::layout::layout_select_effect`/`layout_clear_selection_effect`
// redispatch instead of a `LayoutConfigMutation::SetSelection`.

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutConfig, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = if blueprint { &config.camera } else { &config.preview_camera };
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

#[allow(clippy::too_many_arguments)]
fn hit_test_at(doc: &LayoutSnapshot, config: &LayoutConfig, sx: f64, sy: f64, width: f64, height: f64, blueprint: bool) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, blueprint, sx, sy, width, height);
    let mut engine = LayoutEngine::new();
    // 🕹️ `selected_ids`/`hovered_id` only feed `DisplayRect.selected`/`.hovered` chrome flags, never
    // hit-test correctness — `&[]`/`None` here are harmless (selection/hover are framework-owned now).
    let list = build_display_list_for_page(&mut engine, doc, page, &page.id, &[], None, blueprint);
    list.hit_test(wx as f32, wy as f32)
}
//#endregion 🔖️Shared

//#region 🔖️CanvasPointerDown
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasDragOver
//#endregion 🔖️CanvasDragOver

//#region 🔖️CanvasDragLeave
//#endregion 🔖️CanvasDragLeave

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

//#region 🔖️CanvasDrop
//#endregion 🔖️CanvasDrop

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub surface_id: Option<String>,
    pub button: i64,
    pub extend: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint || payload.button != 0 {
        return Ok(Emit::default());
    }
    let hit = hit_test_at(doc.snapshot, cfg.snapshot, payload.x, payload.y, payload.width, payload.height, blueprint);
    let effect = match hit {
        // ⚖️ `Invertive` merge is the toggle-on-shift-click `extend` used to hand-roll (add if absent,
        // remove if present); `Replace` is a plain click. Both now resolve against the CURRENT
        // framework-owned selection inside `dispatch_interaction_action`, not a locally-read snapshot.
        Some(id) => crate::editor::layout::layout_select_effect(std::slice::from_ref(&id), if payload.extend { "invertive" } else { "replace" }),
        None => crate::editor::layout::layout_clear_selection_effect(),
    };
    Ok(Emit::effect(effect))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::commands::{canvas_drag_leave, canvas_drag_over, canvas_drop, canvas_pointer_move, set_camera};
    use crate::editor::layout::testkit::{dispatch, layout_app, render, test_screen_point};
    use crate::editor::layout::{LayoutCommand, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_SURFACE_PREVIEW};
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::{CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID};

    #[test]
    fn set_camera_mutates_config_and_emits_no_operations() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection");
        let result = dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } }));
        assert!(result.mutations.is_empty(), "camera is a config action and emits no operations");
        assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
    }

    #[test]
    fn set_camera_preview_surface_updates_independently_of_blueprint() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_PREVIEW.into()), camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 } }));
        let preview_json = render(&mut app, crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_BODY_PREVIEW);
        assert!(preview_json.contains(r#""cameraX":3.0"#), "preview scene reflects config camera: {preview_json}");
        let blueprint_json = render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(blueprint_json.contains(r#""cameraX":0.0"#), "blueprint surface camera stays independent: {blueprint_json}");
    }

    /// 🕹️ Selection is framework-owned now: a hit no longer mutates config synchronously, it asks the
    /// host to redispatch `interactionSelect` (`dispatch_interaction_action` runs that on the SAME
    /// instance next, out of band — the test harness doesn't simulate the round trip, so this only
    /// asserts the requested effect is shaped correctly, not that selection state landed).
    #[test]
    fn pointer_down_requests_a_select_effect_for_the_hit_frame() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert!(result.mutations.is_empty(), "pointer down never mutates the document directly");
        let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
        let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
        let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
        assert_eq!(args["domainId"], "elements");
        assert_eq!(args["merge"], "replace");
        assert!(args["targets"].as_str().expect("targets json").contains("frame-image-1"));
    }

    #[test]
    fn pointer_down_extend_click_requests_an_invertive_merge() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: true, x: sx, y: sy, width: 800.0, height: 600.0 }));
        let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
        let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
        let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
        assert_eq!(args["merge"], "invertive");
    }

    #[test]
    fn pointer_down_on_empty_space_requests_clear_selection() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 5.0, 5.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerDown(CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == CLEAR_SELECTION_ACTION_ID)));
    }

    #[test]
    fn pointer_move_requests_a_hover_effect_for_the_hit_frame() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert!(result.mutations.is_empty(), "hover never mutates the document directly");
        let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_HOVER_ACTION_ID)).expect("interactionHover effect");
        let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
        let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("hover args");
        assert!(args["targets"].as_str().expect("targets json").contains("frame-text-1"));
    }

    #[test]
    fn canvas_drop_adds_frame_at_world_coords() {
        let mut app = layout_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 100.0, 200.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert_eq!(result.mutations.len(), 1);
        let doc = app.snapshot().expect("projection");
        let frame = doc.pages[0].frames.last().unwrap();
        let bounds = frame.bounds();
        assert!((bounds.x - 100.0).abs() < 0.01);
        assert!((bounds.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn canvas_drop_page_kind_adds_page() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages.len();
        let result = dispatch(&mut app, LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "page".into(), x: 0.0, y: 0.0, width: 800.0, height: 600.0 }));
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(app.snapshot().expect("projection").pages.len(), before + 1);
    }

    #[test]
    fn drag_over_emits_ghost_and_leave_clears() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 }));
        assert!(render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));

        dispatch(&mut app, LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {}));
        assert!(!render(&mut app, crate::editor::layout::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));
    }
}
//#endregion 🧪️Tests
