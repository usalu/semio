//! 🌊️ Flow viewer — the Main window: a read-only render of the node-graph canvas, the same
//! `SurfaceKind::NodeGraph` scene shape the mutation-capable Main window renders (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2) — built only from the `flow` kernel
//! crate's own pure/session helpers and this artifact's own pure `FlowSnapshot` projection; this file
//! itself imports nothing from the sibling mutation-capable module (`policyViewerPurityBreaches`
//! forbids it outright).

use crate::schema::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
use crate::FlowSnapshot;
use flow::{flow_backed_node_graph_extras, FLOW_LOD_MODE_AUTOMATIC};
use semio_os_infinite_directed_presentation::DagHostSnapshot;
use semio_plugin::scene_surface;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::NodeGraphScene;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_os_kernel::Viewport2d;
use semio_ui_contract::SurfaceKind as ContractSurfaceKind;
use semio_ui_declaration::ui::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "flow-view-main";
pub const BODY_KEY: &str = "flow.view.main";
const SURFACE_ID: &str = "flow.view.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::flow::create_flow_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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

fn dag_host_snapshot_to_workflow(host_snapshot: &DagHostSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = host_snapshot
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: port.id.clone(), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: port.id.clone(), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = host_snapshot
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

/// 👁️ Projects the original composed scene with the viewer canvas defaults.
pub fn render(document: &FlowSnapshot) -> UiAssemblyResult<BuiltNode> {
    let live=document.read_source().map_err(|error|semio_plugin::PluginAssemblyError::new("flow.source",error.into_message()))?;
    let (nodes,edges)=dag_host_snapshot_to_workflow(&flow::FlowHost::project_dag_snapshot(&live,&flow::flow_neuron_kind_info_map()));
    let viewport = Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 };
    let snapshot_json = Some(semio_framework_pack_json::to_json_string(document));
    let mut flow_extras = flow_backed_node_graph_extras(&live, FLOW_LOD_MODE_AUTOMATIC, FLOW_DEFAULT_PROXIMITY_DISTANCE, true, false, FLOW_DEFAULT_GRID_FACTOR, None);
    flow_extras.status_json=Some(flow::initial_flow_status_json(&live));
    let scene = NodeGraphScene {
        editable: Some(false),
        capabilities_json: flow_extras.capabilities_json,
        lod_json: flow_extras.lod_json,
        host_snapshot_json: flow_extras.host_snapshot_json.or(snapshot_json),
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

//#endregion 🧪️Tests
