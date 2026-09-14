//! 🗂️ Edit-mode tool — Reorganize: the wires graph laid out by the framework's stepped force-layout run
//! (`semio-framework-graph-layout-run`). Every iteration is one visible unit: node moves are provisional `move-node`
//! ops the canvas renders from the ToolRun overlay, each node carries an entity trace verdict, and finalize publishes
//! one edit with one move per moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3).

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use dsl::DslValue;
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume};
use semio_framework_plugin::{Fault, LocalizedLabel, ToolDefinition, ToolRunJob};
use semio_framework_tool_run::{JobKindId, ToolRunIdentity};
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "reasoning.wires.layoutRun";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::wires::create_wires_app`; the run vocabulary and its EN/DE
/// labels are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB))), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
}
//#endregion 🔖️Definition

//#region 🔖️Graph
/// 🕸️ The layout graph of a wires document plus the board node id behind every layout node index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WiresLayoutGraph {
    pub graph: LayoutRunGraph,
    pub node_ids: Vec<String>,
}

fn number(entity: &DslValue, key: &str) -> Option<f64> {
    entity.get(key).and_then(DslValue::as_f64).filter(|value| value.is_finite())
}

fn visible(entity: &DslValue) -> bool {
    match entity.get("hidden").and_then(DslValue::as_bool) {
        Some(hidden) => !hidden,
        None => entity.get("visible").and_then(DslValue::as_bool).unwrap_or(true),
    }
}

fn repulsion_radius(node: &DslValue) -> f64 {
    if node.get("shape").and_then(DslValue::as_str) == Some("rectangle") {
        let (width, height) = (number(node, "width").unwrap_or(40.0), number(node, "height").unwrap_or(40.0));
        return ((width * width + height * height).sqrt() * 0.5).max(8.0);
    }
    number(node, "radius").filter(|radius| *radius > 0.0).unwrap_or(32.0)
}

/// 🗺️ Maps the visible board: one body per first-seen node id (committed position as origin, locked placed nodes
/// pinned), one unit spring per distinct node pair whose visible edge endpoints resolve through handles or node ids.
pub fn layout_graph(snapshot: &WiresSnapshot) -> WiresLayoutGraph {
    let scene = crate::wires_working_scene(snapshot);
    let (mut layout, mut index, mut handles) = (WiresLayoutGraph::default(), HashMap::new(), HashMap::new());
    for node in scene.nodes.iter().filter(|node| visible(node)) {
        let Some(id) = node.get("id").and_then(DslValue::as_str) else { continue };
        if index.contains_key(id) {
            continue;
        }
        let at = layout.node_ids.len() as u32;
        index.insert(id.to_string(), at);
        for handle in node.get("handles").and_then(DslValue::as_array).unwrap_or(&[]).iter().filter(|handle| visible(handle)) {
            if let Some(handle_id) = handle.get("id").and_then(DslValue::as_str) {
                handles.insert(handle_id.to_string(), at);
            }
        }
        let origin = number(node, "x").zip(number(node, "y")).map(|(x, y)| LayoutRunPoint::new(x, y));
        let pinned = origin.is_some() && node.get("locked").and_then(DslValue::as_bool) == Some(true);
        layout.graph.nodes.push(LayoutRunNode { entity: layout_run_entity(id), origin, radius: repulsion_radius(node), pinned, anchor: None });
        layout.node_ids.push(id.to_string());
    }
    let resolve = |endpoint: Option<&str>| endpoint.and_then(|endpoint| handles.get(endpoint).or_else(|| index.get(endpoint)).copied());
    let mut seen = BTreeSet::new();
    for edge in scene.edges.iter().filter(|edge| visible(edge)) {
        let (Some(source), Some(target)) = (resolve(edge.get("source").and_then(DslValue::as_str)), resolve(edge.get("target").and_then(DslValue::as_str))) else { continue };
        if source != target && seen.insert((source.min(target), source.max(target))) {
            layout.graph.edges.push(LayoutRunEdge { source: source.min(target), target: source.max(target), weight: 1.0 });
        }
    }
    layout
}
//#endregion 🔖️Graph

//#region 🔖️Job
/// ✍️ Encodes one layout move as this artifact's `move-node` op.
pub fn move_encoder(node_ids: Vec<String>) -> impl LayoutRunOpEncoder + 'static {
    move |node: u32, _entity: u64, position: LayoutRunPoint| protocol::OpBinary::encode_op(&crate::mutations::move_node(node_ids[node as usize].clone(), position.x, position.y)).map_err(|error| LayoutRunEncodeError(format!("{error:?}")))
}

/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional moves folded in.
fn overlay_positions(layout: &WiresLayoutGraph, provisional: &[WiresMutation]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = provisional.iter().filter_map(|op| match op {
        WiresMutation::MoveNode(payload) => index.get(payload.node_id.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.new_x, payload.new_y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the run's base; a settings change resumes it from the ledger's checkpoint with the
/// provisional moves folded over the base positions (`📓️wave-W3-F.md` §3.3).
pub fn build_job(identity: ToolRunIdentity, snapshot: &WiresSnapshot, checkpoint: Option<&[u8]>, provisional: &[WiresMutation]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(snapshot);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, provisional), provisional_len: provisional.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, LayoutRunConfig::default(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| Fault::from(format!("reasoning.wires.layout-run: {error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
