//! 🕸️ Equation play app — the graph window: the editable node-graph canvas.

use crate::editor::equation::workflow_json;
use crate::EquationGraph;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, NodeGraphScene, Viewport2d, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

#[path = "🎚️config/🦀️.rs"]
pub mod config;
use config::EquationCamera;

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GRAPH: &str = "math-graph";
pub const MATH_PLAY_BODY_GRAPH: &str = "equation.play.graph";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::equation::create_equation_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: MATH_PLAY_WINDOW_GRAPH.into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        body_key: MATH_PLAY_BODY_GRAPH.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "math-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(graph: &EquationGraph, camera: &EquationCamera) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges) = workflow_json(graph);
    let viewport = Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom };
    semio_framework_plugin::scene_surface(MATH_PLAY_BODY_GRAPH, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
