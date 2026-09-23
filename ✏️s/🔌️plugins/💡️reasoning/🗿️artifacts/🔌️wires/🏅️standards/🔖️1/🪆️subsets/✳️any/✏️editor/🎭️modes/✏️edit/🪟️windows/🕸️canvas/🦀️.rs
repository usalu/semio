//! 🕸️ Wires play app — the canvas window: the editable WIRES mindmap board.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use dsl::os_pack::json::Value;
use dsl::DslValue;
use semio_framework_plugin::{BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WIRES_PLAY_WINDOW_CANVAS: &str = "reasoning-wires-composite";
pub const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::wires::create_wires_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WIRES_PLAY_WINDOW_CANVAS.into(),
        label: LocalizedLabel::native("Canvas", "Leinwand"),
        body_key: WIRES_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "git-branch".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_wires_app`'s `.window_kind_interactions(..)` call — the
        // "graph" domain (node/edge selection + hover, ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(board: &DslValue, wires: &DslValue, window: &config::WiresCanvasWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let camera_x = window.camera.x;
    let camera_y = window.camera.y;
    let zoom = window.camera.zoom;
    let layers = crate::schema::wires_canvas_layers(board, wires);
    semio_framework_plugin::scene_surface(WIRES_PLAY_SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x, camera_y, zoom, layers_json: dsl::os_pack::json::to_string(&Value::Array(layers)), snapshot: None, tool_run_trace: None, lanes: Vec::new() })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
