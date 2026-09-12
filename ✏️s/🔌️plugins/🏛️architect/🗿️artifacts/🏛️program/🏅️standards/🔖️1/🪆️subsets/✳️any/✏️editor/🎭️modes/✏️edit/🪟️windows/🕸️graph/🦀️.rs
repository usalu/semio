//! 🕸️ Architect graph window — the program elements and their adjacencies as an undirected
//! node-graph surface, laid out on a circle.

use crate::standards::v1::subsets::any::schema::inferences::undirected_edges;
use crate::ProgramSnapshot;
use semio_framework_plugin::{LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, SurfaceKind, WindowKindDefinition, WindowOptions};

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_GRAPH: &str = "architect-graph";
pub const ARCHITECT_BODY_GRAPH: &str = "architect.graph";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_GRAPH.into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        body_key: ARCHITECT_BODY_GRAPH.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "architect-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_architect_app`'s `.window_kind_interactions(..)` call —
        // the "program" domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn graph_media_json(program: &ProgramSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let count = program.elements.len().max(1);
    let radius = 220.0;
    let center_x = 320.0;
    let center_y = 240.0;
    let nodes: Vec<NodeGraphNodeRecord> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            NodeGraphNodeRecord {
                id: element.header.id.to_string(),
                label: Some(element.header.name.clone()),
                x: center_x + radius * angle.cos(),
                y: center_y + radius * angle.sin(),
                width: 108.0,
                height: 44.0,
                inputs: vec![NodeGraphPortRecord { id: "in".into(), label: None, ..Default::default() }],
                outputs: vec![NodeGraphPortRecord { id: "out".into(), label: None, ..Default::default() }],
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = undirected_edges(program)
        .iter()
        .enumerate()
        .map(|(index, (source, target, weight))| NodeGraphEdgeRecord {
            id: format!("edge-{index}"),
            source_node_id: source.to_string(),
            source_port_id: "out".into(),
            target_node_id: target.to_string(),
            target_port_id: "in".into(),
            label: Some(format!("{weight:.1}")),
        })
        .collect();
    (nodes, edges)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `render` carries no `InteractionView`
/// and `NodeGraphScene` has no `interaction_domain` field the wrapper could stamp post-render either
/// (unlike `UiNode::Tree`) — `selection`/`hover` are left at `NodeGraphScene::base`'s defaults
/// (empty/none), matching `dag`'s main window's and `space`'s workflow window's identical gap.
pub fn render(program: &ProgramSnapshot, cfg: &config::ArchitectGraphWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (nodes, edges) = graph_media_json(program);
    let scene = NodeGraphScene { editable: Some(true), capabilities_json: Some(r#"{"directedness":"undirected"}"#.into()), ..NodeGraphScene::base(nodes, edges, cfg.viewport.clone()) };
    semio_framework_plugin::scene_surface(ARCHITECT_BODY_GRAPH, semio_framework_ui_contract::SurfaceKind::NodeGraph, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
