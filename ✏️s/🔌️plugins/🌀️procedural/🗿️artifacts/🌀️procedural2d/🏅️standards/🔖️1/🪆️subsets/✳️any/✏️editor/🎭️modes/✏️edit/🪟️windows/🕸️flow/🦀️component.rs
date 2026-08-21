//! 🕸️ Procedural2d play app — the main node-graph window: the editable flow canvas.

use crate::artifacts::procedural2d::schema::{fixture_to_workflow, host_from_fixture};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_WINDOW_MAIN: &str = "procedural2d-main";
pub const PROCEDURAL2D_PLAY_BODY_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_SURFACE_MAIN: &str = "procedural2d.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL2D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Fluss"),
        body_key: PROCEDURAL2D_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "flow-graph".into(),
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
pub async fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, session: &FlowEvalSession) -> UiNode {
    let fixture = &document.fixture;
    let host = host_from_fixture(fixture);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let flow_extras = flow_backed_node_graph_extras(fixture, "", 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    // 🕹️ `render` carries no `InteractionView` and `NodeGraphScene` has no `interaction_domain` field
    // for the wrapper to stamp post-render either (see the `🧊️3d` sibling window's identical note,
    // ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `selection` is left at
    // `NodeGraphScene::base`'s empty default until a future wave threads interaction into rendering.
    build_node_graph_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: flow_extras.operators,
            catalogue_json: flow_extras.catalogue_json,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_main_graph_scene() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_MAIN).contains("node-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
        let mut app = app();
        let json = render_body(&mut app, PROCEDURAL2D_PLAY_BODY_MAIN);
        let value: serde_json::Value = serde_json::from_str(&json).expect("ui node json");
        let graph = value.get("nodeGraph").expect("nodeGraph");
        assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
        assert!(graph.get("operators").and_then(|v| v.as_array()).is_some_and(|items| !items.is_empty()));
        assert!(graph.get("capabilitiesJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow")));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PROCEDURAL2D_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }
}
//#endregion 🧪️Tests
