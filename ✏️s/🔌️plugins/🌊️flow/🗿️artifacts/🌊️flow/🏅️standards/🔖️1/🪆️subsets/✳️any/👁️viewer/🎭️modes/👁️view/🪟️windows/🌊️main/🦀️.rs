//! 🌊️ Flow viewer — the Main window: a read-only render of the node-graph canvas, the same
//! `SurfaceKind::NodeGraph` scene shape the mutation-capable Main window renders (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2) — built only from the `flow` kernel
//! crate's own pure/session helpers and this artifact's own pure `FlowSnapshot` projection; this file
//! itself imports nothing from the sibling mutation-capable module (`policyViewerPurityBreaches`
//! forbids it outright).

use crate::artifacts::flow::schema::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
use crate::artifacts::flow::FlowSnapshot;
use flow::{dag::DagFixture, flow_backed_node_graph_extras, flow_host_with_session, FlowEvalSession, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "flow-view-main";
pub const BODY_KEY: &str = "flow.view.main";
const SURFACE_ID: &str = "flow.view.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::flow::create_flow_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Flow", "Flow"),
        body_key: BODY_KEY.into(),
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
/// 👁️ Own copy of the mutation-capable Main window's identically named helper (duplication is the
/// deliberate cost of a genuinely independent viewer, contract §2.2).
fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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

/// 👁️ Pure `FlowSnapshot -> BuiltNode` read: a fresh, throwaway `FlowEvalSession` built for this call only
/// (never persisted — a viewer has no `Transient`/`Config` lane to hold one), the artifact's own pure
/// LOD/grid/proximity defaults (`Config = NoConfig` means there is no persisted per-session camera or
/// canvas state to read), no selection, no preview-off overlay.
pub fn render(document: &FlowSnapshot) -> UiAssemblyResult<BuiltNode> {
    let live = document.to_fixture();
    let session = FlowEvalSession::new();
    let host = flow_host_with_session(&live, &session);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let fixture_json = Some(dsl::os_pack::json::to_json_string(document));
    let flow_extras = flow_backed_node_graph_extras(&live, FLOW_LOD_MODE_AUTOMATIC, FLOW_DEFAULT_PROXIMITY_DISTANCE, true, false, FLOW_DEFAULT_GRID_FACTOR, Some(&session));
    let scene = NodeGraphScene {
        editable: Some(false),
        operators: flow_extras.operators,
        capabilities_json: flow_extras.capabilities_json,
        lod_json: flow_extras.lod_json,
        fixture_json: flow_extras.fixture_json.or(fixture_json),
        eval_json: flow_extras.eval_json,
        status_json: flow_extras.status_json,
        selection: Vec::new(),
        preview_off_json: None,
        ..NodeGraphScene::base(nodes, edges, viewport)
    };
    scene_surface(SURFACE_ID, ContractSurfaceKind::NodeGraph, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BODY_KEY);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_node_graph_scene_for_the_default_document() {
        let document = FlowSnapshot::default();
        let node = render(&document).expect("render");
        let json = serde_json::to_string(&node).expect("render json");
        assert!(json.contains("node-graph"));
    }
}
//#endregion 🧪️Tests
