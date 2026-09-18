//! 🕸️ WFC `wfc-graph` window — the slot/adjacency canvas, on `SurfaceKind::NodeGraph`.
//!
//! ⚠️ **THIS FILE IS COPIED VERBATIM BY `wfc3d` (slice A5).** It is written to be copyable: it knows
//! NOTHING about `Wfc2dSnapshot`. Its whole view of a document is the local `SlotGraphView` trait
//! plus the two plain `GraphSlotView`/`GraphEdgeView` structs below — "slots with id/x/y/width/
//! height/pinned, and edges between them". `wfc3d` implements the same trait over `Wfc3dSnapshot`
//! (projecting `x`/`y` and dropping `z`) and copies this file unchanged, keeping the SAME window
//! kind id, the SAME body key constant names and the SAME action ids, so both artifacts present one
//! window to the shell. Anything artifact-specific belongs in the caller, never here.
//!
//! Gesture → mutation contract (the caller wires it; this file only declares the actions):
//! - dragging a node → ONE `move-slot` per gesture, never one per pointer tick (mid-drag frames ride
//!   the window transient, per "Per-Frame State Belongs In Window Transient");
//! - a connect gesture between two ports → ONE `connect-slots` carrying a fresh edge id;
//! - selecting a node and invoking `pin-slot`/`unpin-slot` → the pin verbs, using the pane's own
//!   armed tile (`Wfc2dConfig::active_tile_id`).
//!
//! Every node carries exactly one input port `adjacent-in` and one output port `adjacent-out`: the
//! adjacency graph is undirected, so the two ports exist only to give the canvas something to drag a
//! wire between. Edge direction on screen is the authored `from`→`to`, which the solver symmetrises.

use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::{
    scene_surface, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions,
};
use store::Viewport2d;

//#region 🔖️Constants
/// 🪪️ The window kind id both `wfc2d` and `wfc3d` declare — one window kind, two artifacts.
pub const WFC_GRAPH_WINDOW: &str = "wfc-graph";
pub const WFC_GRAPH_BODY: &str = "wfc.graph";
const WFC_GRAPH_SURFACE: &str = "wfc.graph";
/// 🔌️ The single in/out port pair every slot node carries.
pub const WFC_GRAPH_PORT_IN: &str = "adjacent-in";
pub const WFC_GRAPH_PORT_OUT: &str = "adjacent-out";
//#endregion 🔖️Constants

//#region 🔖️View
/// 📍 One slot, as this window needs it — deliberately NOT the artifact's own slot type.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GraphSlotView {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub pinned_tile_id: Option<String>,
}

/// 🔗 One adjacency edge, as this window needs it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GraphEdgeView {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
    pub relation: String,
}

/// 🧩️ The ONLY contract this window has with a document. An artifact implements it by projecting its
/// own slots/edges; nothing else about the snapshot is reachable from here.
pub trait SlotGraphView {
    fn graph_slots(&self) -> Vec<GraphSlotView>;
    fn graph_edges(&self) -> Vec<GraphEdgeView>;
}

/// 🎥️ The pane's camera, passed in rather than read from any particular config type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for GraphCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}
//#endregion 🔖️View

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the artifact's own `create_*_editor`. Every verb is
/// `Migrated`: a `BatchOnly` verb never reaches interactive dispatch and would be dead in the pane.
pub fn definition() -> WindowKindDefinition {
    let mut definition = WindowKindDefinition {
        id: WFC_GRAPH_WINDOW.into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        body_key: WFC_GRAPH_BODY.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "network".into(),
        options: WindowOptions::default(),
        actions: vec![
            ActionDefinition::bounded_catalog("create-slot", LocalizedLabel::native("Create Slot", "Slot erstellen"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("delete-slot", LocalizedLabel::native("Delete Slot", "Slot löschen"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("move-slot", LocalizedLabel::native("Move Slot", "Slot verschieben"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("resize-slot", LocalizedLabel::native("Resize Slot", "Slot skalieren"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("connect-slots", LocalizedLabel::native("Connect Slots", "Slots verbinden"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("disconnect-slots", LocalizedLabel::native("Disconnect Slots", "Slots trennen"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("pin-slot", LocalizedLabel::native("Pin Slot", "Slot festlegen"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("unpin-slot", LocalizedLabel::native("Unpin Slot", "Slot freigeben"), ActionKind::Mutation),
            ActionDefinition::bounded_catalog("change-seed", LocalizedLabel::native("Change Seed", "Seed ändern"), ActionKind::Mutation),
        ],
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    };
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    }
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Projection
/// 🏷️ A node's caption: the slot id, plus the pinned tile when the author fixed one. Kept short on
/// purpose — a window body's owned text is capacity-bounded, and one oversized admission fails the
/// WHOLE surface refresh, not just this row.
fn node_label(slot: &GraphSlotView) -> String {
    let mut label = match &slot.pinned_tile_id {
        Some(tile) => format!("{} 📌 {tile}", slot.id),
        None => slot.id.clone(),
    };
    if label.len() > 96 {
        label.truncate(93);
        label.push_str("...");
    }
    label
}

fn port(node_id: &str, port_id: &str, label: &str) -> NodeGraphPortRecord {
    NodeGraphPortRecord { id: format!("{node_id}@{port_id}"), label: Some(label.to_string()), ..Default::default() }
}

/// 🕸️ Projects a slot graph onto the canvas' own record types — one node per slot at its authored
/// rectangle, one edge per adjacency.
pub fn graph_records(view: &impl SlotGraphView) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let slots = view.graph_slots();
    let nodes: Vec<NodeGraphNodeRecord> = slots
        .iter()
        .map(|slot| NodeGraphNodeRecord {
            id: slot.id.clone(),
            label: Some(node_label(slot)),
            x: slot.x,
            y: slot.y,
            width: if slot.width > 0.0 { slot.width } else { 1.0 },
            height: if slot.height > 0.0 { slot.height } else { 1.0 },
            inputs: vec![port(&slot.id, WFC_GRAPH_PORT_IN, "adjacent")],
            outputs: vec![port(&slot.id, WFC_GRAPH_PORT_OUT, "adjacent")],
            icon: Some("square-dashed".into()),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = view
        .graph_edges()
        .iter()
        .map(|edge| NodeGraphEdgeRecord {
            id: edge.id.clone(),
            source_node_id: edge.from_slot_id.clone(),
            source_port_id: format!("{}@{}", edge.from_slot_id, WFC_GRAPH_PORT_OUT),
            target_node_id: edge.to_slot_id.clone(),
            target_port_id: format!("{}@{}", edge.to_slot_id, WFC_GRAPH_PORT_IN),
            label: (!edge.relation.is_empty()).then(|| edge.relation.clone()),
        })
        .collect();
    (nodes, edges)
}
//#endregion 🔖️Projection

//#region 🔖️Render
/// 🕹️ Renders the editable slot graph. `selection` is the ids the shell currently holds picked, so a
/// `pin-slot`/`delete-slot` invocation has a target without the window owning selection state.
pub fn render(view: &impl SlotGraphView, camera: GraphCamera, selection: &[String]) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges) = graph_records(view);
    let viewport = Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom };
    scene_surface(
        WFC_GRAPH_SURFACE,
        semio_framework_ui_contract::SurfaceKind::NodeGraph,
        &NodeGraphScene { editable: Some(true), selection: selection.to_vec(), ..NodeGraphScene::base(nodes, edges, viewport) },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
