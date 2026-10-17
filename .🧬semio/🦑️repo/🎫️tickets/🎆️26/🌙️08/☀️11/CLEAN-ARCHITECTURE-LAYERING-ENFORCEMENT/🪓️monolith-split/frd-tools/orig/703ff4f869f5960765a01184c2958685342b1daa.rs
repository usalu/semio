//! 🕒️ Canvas2d camera laws that drive the Scenes standalone emulator: the settled wheel-zoom and pan-drag dispatch.
//!
//! 🧭️ They name `handle_scene_wheel` and `handle_scene_pointer_move`, the test-only emulator of the Scenes standalone law, so they are mounted next to it by the Scenes companion.

use super::{handle_scene_pointer_move, handle_scene_wheel};
use semio_framework_os_renderer_wgpu_host_base::app_now_ms;
use semio_framework_os_renderer_wgpu_scene_state::{SceneDrag, SceneDragMode, camera_fixture_grant, camera_fixture_lease, clear_camera_fixture, mutate_scene_state, sweep_expired_scene_camera_dispatches};
use serde_json::Value;
use ui_wgpu::wgpu::{Rect, SurfaceKind, UiComponentSceneNode, UiPresence};

fn canvas_scene(surface_id: &str, layers_json: String) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::Canvas2d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: Some(semio_framework_ui_scene_plane::Canvas2dScene { framing: None, camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }),
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

fn canvas_camera_gesture_fixture() -> Value {
    serde_json::from_str(runtime_fixture_text("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).expect("shared Canvas2d camera gesture fixture")
}

/// 🕒️ A Canvas2d wheel tick mutates the viewport immediately but only SCHEDULES its `setCamera` —
/// nothing is due yet, and nothing is dispatched, until the deadline sweep says otherwise.
#[test]
fn canvas2d_wheel_schedules_a_settled_camera_dispatch_without_firing_immediately() {
    let _camera_owner = camera_fixture_lease(camera_fixture_grant());
    let surface_id = "wheel-settle-canvas2d";
    let node = canvas_scene(surface_id, "[]".to_string());
    let actions = handle_scene_wheel(&node, Rect::new(0.0, 0.0, 400.0, 300.0), 50.0, 50.0, -100.0, false);
    assert!(actions.is_empty(), "wheel-zoom never dispatches inline anymore");
    let immediate = sweep_expired_scene_camera_dispatches(app_now_ms());
    assert!(immediate.iter().all(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) != Some(surface_id)), "sweeping immediately must not yet report this surface");
    let due = sweep_expired_scene_camera_dispatches(app_now_ms() + canvas_camera_gesture_fixture()["wheel"]["settleDelayMs"].as_f64().unwrap());
    let matched = due.iter().find(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) == Some(surface_id)).expect("this surface's setCamera fires once its deadline has passed");
    assert_eq!(matched.controller_id, "controller");
    assert_eq!(matched.action, "setCamera");
}

/// 🕒️ A Canvas2d pan-drag (`SceneDragMode::PanViewport`) gets the identical settle-then-dispatch
/// treatment as wheel-zoom — same deadline map, same sweep.
///
/// 🧹️ The settle map is process-wide and the sweep drains EVERY surface's deadline, so this law
/// clears it first; otherwise a neighbouring scene's armed camera is counted here
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
#[test]
fn canvas2d_pan_drag_schedules_a_settled_camera_dispatch() {
    let _camera_owner = camera_fixture_lease(camera_fixture_grant());
    clear_camera_fixture(camera_fixture_grant());
    let surface_id = "pan-settle-canvas2d";
    let node = canvas_scene(surface_id, "[]".to_string());
    mutate_scene_state(surface_id, |state| {
        state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
    });
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let actions = handle_scene_pointer_move(&node, bounds, 60.0, 60.0, true, 1, 5.0, 5.0);
    assert!(
        actions.iter().all(|action| action.action != "setCamera"),
        "the pan itself never dispatches setCamera inline, even though Canvas2d's own \
         canvasPointerMove tracking action still fires alongside it"
    );
    let due = sweep_expired_scene_camera_dispatches(app_now_ms() + 400.0);
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(surface_id));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
