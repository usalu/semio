//! 🌊️ Flow play app — the main node-graph window: the editable flow canvas.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::editor::flow::host_from_snapshot;
use crate::editor::flow::modes::edit::windows::main::options;
use crate::editor::flow::terminology::FlowPlayLabels;
use crate::FlowSnapshot;
use flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_artifact_infinite_dag::DagFixture;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use semio_framework::Viewport2d;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene};

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
pub const FLOW_PLAY_BODY_MAIN: &str = "flow.play.main";
const FLOW_PLAY_SURFACE_MAIN: &str = "flow.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::flow::create_flow_app`. `options.measures` stays
/// empty here on purpose: flow's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Flow"),
        body_key: FLOW_PLAY_BODY_MAIN.into(),
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

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
pub fn window_measures(config: &FlowMainWindowConfig, labels: &FlowPlayLabels) -> Vec<WindowMeasure> {
    vec![options::lod::measure(config, labels), options::proximity::measure(config, labels), options::grid::measure(config, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Workflow
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}
//#endregion 🔖️Workflow

//#region 🔖️Render
pub fn render(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> UiAssemblyResult<BuiltNode> {
    let host = host_from_snapshot(fixture, config, session);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = Viewport2d { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let fixture_json = Some(dsl::json::to_json_string(&fixture.to_fixture()));
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "graph" domain's live selection
    // is framework-owned `InteractionState` now, and `ArtifactApp::render` is not threaded an
    // `InteractionView` this wave — the scene's selection payload drops to empty rather than showing
    // stale app-local state (a real known gap, mirrors lowpoly's identical `render`/status-line note).
    let selection: Vec<String> = Vec::new();
    let flow_extras = flow_backed_node_graph_extras(&fixture.to_fixture(), &config.lod_mode, config.proximity_distance, config.grid_visible, config.grid_snap_enabled, config.grid_factor, Some(session));
    let preview_off_json = if config.preview_off_node_ids.is_empty() { None } else { serde_json::to_string(&config.preview_off_node_ids).ok() };
    let scene = NodeGraphScene {
        editable: Some(true),
        capabilities_json: flow_extras.capabilities_json,
        lod_json: flow_extras.lod_json,
        fixture_json: flow_extras.fixture_json.or(fixture_json),
        eval_json: flow_extras.eval_json,
        status_json: flow_extras.status_json,
        selection,
        preview_off_json,
        ..NodeGraphScene::base(nodes, edges, viewport)
    };
    scene_surface(FLOW_PLAY_SURFACE_MAIN, ContractSurfaceKind::NodeGraph, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
