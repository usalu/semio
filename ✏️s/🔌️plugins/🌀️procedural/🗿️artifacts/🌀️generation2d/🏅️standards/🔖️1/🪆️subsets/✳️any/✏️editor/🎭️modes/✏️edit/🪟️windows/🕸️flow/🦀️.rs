//! 🕸️ Generation2d play app — the main node-graph window: the editable flow canvas.

use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use crate::editor::generation2d::{
    GENERATION2D_GRAPH_EDGE_TARGET_PREFIX, GENERATION2D_GRAPH_HANDLE_TARGET_PREFIX, GENERATION2D_GRAPH_NODE_TARGET_PREFIX, GENERATION2D_INTERACTION_DOMAIN,
};
use crate::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use crate::Generation2dSnapshot;
use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, NodeGraphScene, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui::wgpu::{NodeGraphInteractionDomain, NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord};

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_MAIN: &str = "generation2d-main";
pub const GENERATION2D_PLAY_BODY_MAIN: &str = "generation2d.play.main";
const GENERATION2D_PLAY_SURFACE_MAIN: &str = "generation2d.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION2D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Fluss"),
        body_key: GENERATION2D_PLAY_BODY_MAIN.into(),
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

//#region 🔖️Operators
/// 🔌️ Channel code the operator catalogue speaks — `{nodeId}@{portId}` on a scene port, bare `portId` on a kind.
fn operator_channel_code(port_id: &str) -> &str {
    port_id.rsplit_once('@').map(|(_, code)| code).filter(|code| !code.is_empty()).unwrap_or(port_id)
}

fn operator_channel(code: &str, label: &str) -> NodeGraphOperatorChannelRecord {
    NodeGraphOperatorChannelRecord {
        code: code.to_string(),
        abbreviation: code.to_string(),
        name: label.to_string(),
        full_name: label.to_string(),
        operators: Vec::new(),
        value_types: Vec::new(),
        default_json: None,
        label: Some(label.to_string()),
        cardinality: "!".into(),
    }
}

fn kind_extension_name(kind: &str) -> (String, String) {
    match kind.rsplit_once('.') {
        Some((extension, name)) => (extension.to_string(), name.to_string()),
        None => (String::new(), kind.to_string()),
    }
}

/// 🛍️ Document-derived operator records — one per neuron KIND the open graph actually holds.
fn document_operator_records(dag: &semio_framework_artifact_infinite_dag::DagHostSnapshot, nodes: &[NodeGraphNodeRecord]) -> Vec<NodeGraphOperatorRecord> {
    let catalogue = semio_framework_os_flow::flow_operator_catalogue_records();
    let node_by_id: std::collections::BTreeMap<&str, &NodeGraphNodeRecord> = nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut kinds: Vec<String> = Vec::new();
    for spec in &dag.nodes {
        let Some(kind) = spec.operator_kind.as_deref().filter(|kind| !kind.is_empty() && !kind.starts_with("core.")) else { continue };
        if !kinds.iter().any(|known| known == kind) {
            kinds.push(kind.to_string());
        }
    }
    kinds
        .into_iter()
        .map(|kind| {
            if let Some(record) = catalogue.iter().find(|record| record.id == kind) {
                return record.clone();
            }
            let mut inputs = Vec::new();
            let mut outputs = Vec::new();
            let mut seen_in = std::collections::BTreeSet::new();
            let mut seen_out = std::collections::BTreeSet::new();
            for spec in &dag.nodes {
                if spec.operator_kind.as_deref() != Some(kind.as_str()) {
                    continue;
                }
                let Some(node) = node_by_id.get(spec.id.as_str()) else { continue };
                for port in &node.inputs {
                    let code = operator_channel_code(&port.id).to_string();
                    if seen_in.insert(code.clone()) {
                        inputs.push(operator_channel(&code, port.label.as_deref().unwrap_or(&code)));
                    }
                }
                for port in &node.outputs {
                    let code = operator_channel_code(&port.id).to_string();
                    if seen_out.insert(code.clone()) {
                        outputs.push(operator_channel(&code, port.label.as_deref().unwrap_or(&code)));
                    }
                }
            }
            let (extension, name) = kind_extension_name(&kind);
            NodeGraphOperatorRecord {
                id: kind,
                extension: extension.clone(),
                name: name.clone(),
                abbreviation: name.chars().next().map(|ch| ch.to_uppercase().to_string()).unwrap_or_else(|| "?".into()),
                icon: "box".into(),
                summary: String::new(),
                inputs,
                outputs,
                variadic_input: None,
                variadic_output: None,
                group: if extension.is_empty() { Vec::new() } else { vec![extension] },
            }
        })
        .collect()
}
//#endregion 🔖️Operators

//#region 🔖️Render
/// 🕹️ `selection` is the live `graph`-domain widget ids `render_with_request_context` resolved —
/// the marks-free `render` path passes an empty slice.
pub fn render(document: &Generation2dSnapshot, config: &config::Generation2dMainWindowConfig, session: &FlowEvalSession, selection: &[String]) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let host_snapshot = &document.host_snapshot;
    let (nodes, edges, operators) = with_host(host_snapshot, |host| {
        let (nodes, edges) = dag_host_snapshot_to_workflow(&host.dag.host_snapshot);
        let operators = document_operator_records(&host.dag.host_snapshot, &nodes);
        (nodes, edges, operators)
    });
    let viewport = Viewport2d { x: config.viewport.x, y: config.viewport.y, zoom: config.viewport.zoom };
    let flow_extras = flow_backed_node_graph_extras(host_snapshot, "", 0.0, true, false, semio_framework_ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    let _ = GENERATION2D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION2D_PLAY_SURFACE_MAIN,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::NodeGraph,
        &NodeGraphScene {
            editable: Some(true),
            interaction_domain: Some(NodeGraphInteractionDomain {
                id: GENERATION2D_INTERACTION_DOMAIN.into(),
                node_target_prefix: GENERATION2D_GRAPH_NODE_TARGET_PREFIX.into(),
                edge_target_prefix: GENERATION2D_GRAPH_EDGE_TARGET_PREFIX.into(),
                handle_target_prefix: GENERATION2D_GRAPH_HANDLE_TARGET_PREFIX.into(),
            }),
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            host_snapshot_json: flow_extras.host_snapshot_json,
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            operators,
            selection: selection.to_vec(),
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
