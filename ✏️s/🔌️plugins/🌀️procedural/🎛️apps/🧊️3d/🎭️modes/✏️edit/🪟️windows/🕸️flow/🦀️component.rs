//! 🕸️ Procedural3d play app — the main flow-graph window (edit mode).

use crate::apps::procedural3d::config::Procedural3dConfig;
use crate::apps::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use crate::artifacts::procedural3d::engine::{fixture_to_workflow, host_from_fixture};
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow_core::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphHover, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
pub const PROCEDURAL_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
const PROCEDURAL_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Workflow"),
        body_key: PROCEDURAL_3D_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "flow-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The LOD chrome measure for this window — collected fresh per frame, never frozen into the manifest.
pub fn window_measures(lod_mode: &str, on_change: impl Fn(&str, Option<serde_json::Value>) -> semio_framework_plugin::ActionDescriptor) -> Vec<WindowMeasure> {
    let current = if lod_mode.is_empty() { "medium" } else { lod_mode };
    vec![WindowMeasure::Select {
        id: "procedural3d-measure-lod".into(),
        label: Some("LOD".into()),
        value: current.into(),
        items: vec![
            semio_framework_plugin::MeasureSelectItem { id: "procedural3d-measure-lod-coarse".into(), value: "coarse".into(), label: "Coarse".into() },
            semio_framework_plugin::MeasureSelectItem { id: "procedural3d-measure-lod-medium".into(), value: "medium".into(), label: "Medium".into() },
            semio_framework_plugin::MeasureSelectItem { id: "procedural3d-measure-lod-fine".into(), value: "fine".into(), label: "Fine".into() },
        ],
        on_change: on_change("setLodMode", None),
    }]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Procedural3dDocument, config: &Procedural3dConfig, session: &FlowEvalSession) -> UiNode {
    let fixture = &document.fixture;
    crate::artifacts::procedural3d::engine::sync_flow_extension_contributions(&config.contributions_json);
    let host = host_from_fixture(fixture);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let selection = config.selected_node_ids.clone();
    let flow_extras = flow_backed_node_graph_extras(fixture, &config.lod_mode, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    build_node_graph_scene(
        PROCEDURAL_3D_PLAY_SURFACE_MAIN,
        PROCEDURAL_3D_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: flow_extras.operators,
            catalogue_json: flow_extras.catalogue_json,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            selection,
            hover: config.hovered_node_id.as_ref().map(|id| NodeGraphHover { node_id: Some(id.clone()) }),
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn renders_node_graph_scene() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_MAIN).contains("node-graph"));
    }

    #[test]
    fn main_graph_scene_exports_flow_backed_node_graph_fields() {
        let mut app = app();
        let json = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_MAIN);
        let value: serde_json::Value = serde_json::from_str(&json).expect("ui node json");
        let graph = value.get("nodeGraph").expect("nodeGraph");
        assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
        let operators = graph.get("operators").and_then(|value| value.as_array()).expect("operators array");
        assert!(operators.iter().any(|operator| operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.contains("math.add") || id.contains("brep."))));
        let capabilities = graph.get("capabilitiesJson").and_then(|v| v.as_str()).unwrap_or_default();
        assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    }
}
//#endregion 🧪️Tests
