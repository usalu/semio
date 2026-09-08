//! 🕸️ Equation play app — the graph window: the editable node-graph canvas.

use crate::{EquationCamera, EquationGraph};
use crate::editor::equation::workflow_json;
use semio_framework_plugin::{LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, BuiltNode, UiAssemblyResult, WindowKindDefinition, WindowOptions};

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
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    semio_framework_plugin::scene_surface(MATH_PLAY_BODY_GRAPH, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_node_graph_scene() {
        let graph = EquationGraph::default();
        let camera = EquationCamera::default();
        let node = render(&graph, &camera).expect("graph surface");
        let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("graph must render a surface") };
        let scene: NodeGraphScene = semio_framework_ui_scene::decode(&props).expect("graph payload");
        let (nodes, edges) = workflow_json(&graph);
        assert_eq!(scene.editable, Some(true));
        assert_eq!(scene.nodes, nodes);
        assert_eq!(scene.edges, edges);
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, MATH_PLAY_BODY_GRAPH);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }
}
//#endregion 🧪️Tests
