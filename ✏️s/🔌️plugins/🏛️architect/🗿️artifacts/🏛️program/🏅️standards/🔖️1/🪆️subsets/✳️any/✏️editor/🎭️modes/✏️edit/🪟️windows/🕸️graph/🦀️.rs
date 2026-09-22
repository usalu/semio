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

/// 📐️ One node box, in scene units.
pub const ARCHITECT_GRAPH_NODE_WIDTH: f64 = 108.0;
pub const ARCHITECT_GRAPH_NODE_HEIGHT: f64 = 44.0;
/// ↔️ The smallest gap this layout keeps between two neighbouring node boxes.
pub const ARCHITECT_GRAPH_NODE_GAP: f64 = 24.0;
/// 🖼️ The gutter between the scene origin and the node bounding box. The window's own
/// `ArchitectGraphWindowConfig::default().viewport` is `{x: 0, y: 0, zoom: 1}`, so the scene origin IS
/// the window's top-left corner until the caller pans — every node must therefore live in the
/// positive quadrant, close enough to the origin that an untouched window frames the whole program.
pub const ARCHITECT_GRAPH_MARGIN: f64 = 24.0;
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
/// 📏️ The ring radius `count` node boxes need. One element sits ON the ring centre (radius 0); from
/// two elements up the radius is the smallest one whose neighbour-to-neighbour CHORD still clears a
/// whole node box plus [`ARCHITECT_GRAPH_NODE_GAP`], measured on the box diagonal so the guarantee
/// holds at every angle. A fixed radius (this window used 220 for every program) pushed a two-element
/// program's nodes 440 scene units apart, far outside an untouched window — see
/// [`ARCHITECT_GRAPH_MARGIN`].
pub fn graph_ring_radius(count: usize) -> f64 {
    if count < 2 {
        return 0.0;
    }
    let clearance = ARCHITECT_GRAPH_NODE_WIDTH.hypot(ARCHITECT_GRAPH_NODE_HEIGHT) + ARCHITECT_GRAPH_NODE_GAP;
    clearance / (2.0 * (std::f64::consts::PI / count as f64).sin())
}

pub fn graph_media_json(program: &ProgramSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let count = program.elements.len().max(1);
    let radius = graph_ring_radius(count);
    // 📍 Anchors the node BOUNDING BOX at (margin, margin) so an untouched viewport frames it.
    let center_x = ARCHITECT_GRAPH_MARGIN + radius + ARCHITECT_GRAPH_NODE_WIDTH / 2.0;
    let center_y = ARCHITECT_GRAPH_MARGIN + radius + ARCHITECT_GRAPH_NODE_HEIGHT / 2.0;
    let nodes: Vec<NodeGraphNodeRecord> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            NodeGraphNodeRecord {
                id: element.header.id.to_string(),
                label: Some(element.header.name.clone()),
                x: center_x + radius * angle.cos() - ARCHITECT_GRAPH_NODE_WIDTH / 2.0,
                y: center_y + radius * angle.sin() - ARCHITECT_GRAPH_NODE_HEIGHT / 2.0,
                width: ARCHITECT_GRAPH_NODE_WIDTH,
                height: ARCHITECT_GRAPH_NODE_HEIGHT,
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
