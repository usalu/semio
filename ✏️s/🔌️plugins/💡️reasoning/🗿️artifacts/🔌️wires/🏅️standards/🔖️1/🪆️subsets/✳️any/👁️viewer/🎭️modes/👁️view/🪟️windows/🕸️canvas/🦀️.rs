//! 🕸️ Wires viewer — the canvas window: a read-only render of the WIRES mindmap board. Genuinely
//! independent of the sibling editor module (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! contract §2.2) — never calls into it; duplicates the small, pure 2D-scene projection instead,
//! built only from framework-level `build_canvas_2d_scene`/`Canvas2dScene` and artifact-level pure
//! helpers (`wires_working_board`, `fixture_camera`, `fixture_nodes`, `fixture_edges`,
//! `wires_relationships`, `dsl_to_json`) — the wires board is a general node/edge graph (cycles legal,
//! see the artifact's own `topology.cycle_free` inference), so no SDK window kit (Text/Table/Tree/
//! Image/Mesh/Document/Media) matches it directly; `TreeWindowKit` was checked and rejected for this
//! reason before writing this render function by hand.

use crate::schema::fixture_camera;
use crate::WiresSnapshot;
use dsl::os_pack::json::Value;
use semio_framework_plugin::{BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WIRES_VIEW_WINDOW_CANVAS: &str = "reasoning-wires-view-composite";
pub const WIRES_VIEW_BODY_CANVAS: &str = "reasoning.wires.view.composite";
const WIRES_VIEW_CANVAS_SURFACE_ID: &str = "reasoning.wires.view.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::wires::create_wires_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WIRES_VIEW_WINDOW_CANVAS.into(),
        label: LocalizedLabel::native("Canvas", "Leinwand"),
        body_key: WIRES_VIEW_BODY_CANVAS.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "git-branch".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
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
/// 🖼️ The read-only canvas: the schema's own `wires_canvas_layers` projection — the viewer stays
/// independent of the editor (contract §2.2) by sharing the schema, never an editor window.
pub fn render(document: &WiresSnapshot) -> UiAssemblyResult<BuiltNode> {
    let board = crate::wires_working_board(document);
    let wires = &document.wires_fixture;
    let (camera_x, camera_y, zoom) = fixture_camera(&board);
    let layers = crate::schema::wires_canvas_layers(&board, wires);
    semio_framework_plugin::scene_surface(
        WIRES_VIEW_CANVAS_SURFACE_ID,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x, camera_y, zoom, layers_json: dsl::os_pack::json::to_string(&Value::Array(layers)), snapshot: None, tool_run_trace: None, lanes: Vec::new() },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
