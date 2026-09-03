//! 🕸️ Equation play app — the graph window: the editable node-graph canvas.

use crate::artifacts::equation::{EquationCamera, EquationGraph};
use crate::editor::equation::{empty_component_scene, workflow_json};
use semio_framework_plugin::{LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GRAPH: &str = "math-graph";
pub const MATH_PLAY_BODY_GRAPH: &str = "equation.play.graph";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::equation::create_equation_app`.
pub async fn definition() -> WindowKindDefinition {
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
pub async fn render(graph: &EquationGraph, camera: &EquationCamera) -> UiNode {
    let (nodes, edges) = workflow_json(graph);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let mut scene = empty_component_scene(MATH_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_node_graph_scene() {
        // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
        // `Debug` gives the same "the scene populated its node_graph slot" check the old JSON
        // substring check made, without needing `serde_json` for a framework type.
        let debug = format!("{:?}", render(&EquationGraph::default(), &EquationCamera::default()));
        assert!(debug.contains("node_graph: Some"), "expected a populated node_graph slot: {debug}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, MATH_PLAY_BODY_GRAPH);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }
}
//#endregion 🧪️Tests
