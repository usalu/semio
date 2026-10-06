//! 🗂️ Edit-mode tool — Reorganize: the wires board laid out by the framework's stepped force-layout run
//! (`semio-framework-graph-layout-run`) as a MEMBER run on the composed board child (`ToolRunDefinition::member`, design
//! §12, §20.15). Every iteration is one visible unit: node moves are provisional graph `move-node` ops the canvas renders
//! from the run's composed child read, each node carries an entity trace verdict, and finalize publishes ONE child edit
//! with one move per moved node (`📋️tool-run-contract.md` §3.7, `📓️wave-W3-F.md` §3).

use crate::{GraphNodeId, SemioGraphMutation};
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::move_node::MoveNode;
use semio_framework_graph_layout_run::{layout_run_definition, layout_run_entity, layout_run_job, layout_run_overlay_positions, LayoutRunConfig, LayoutRunEdge, LayoutRunEncodeError, LayoutRunGraph, LayoutRunNode, LayoutRunOpEncoder, LayoutRunPoint, LayoutRunResume};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ToolDefinition;
use semio_framework_plugin::ToolRunJob;
use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunIdentity};
use std::collections::{BTreeSet, HashMap};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reorganize";
/// 🧵️ Job kind of the layout run (`ToolRunDefinition.runJob`).
pub const LAYOUT_RUN_JOB: &str = "reasoning.wires.layoutRun";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧩️ The shared layout run, targeting the composed board child the moves land in.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition { member: Some(crate::WIRES_CONTENT_SLOT.into()), ..layout_run_definition(JobKindId::new(LAYOUT_RUN_JOB)) }
}

/// 🧱️ Stitched into the app manifest by `crate::editor::wires::create_wires_app`; the run vocabulary and its EN/DE
/// labels are the shared layout run's.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(run_definition()), ..::semio_framework_async::poll::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reorganize", "Neu anordnen"), "rotate-cw")) }
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
    entity.get(key).and_then(semio_framework_value::DslValue::as_f64).filter(|value| value.is_finite())
}

fn visible(entity: &DslValue) -> bool {
    match entity.get("hidden").and_then(semio_framework_value::DslValue::as_bool) {
        Some(hidden) => !hidden,
        None => entity.get("visible").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true),
    }
}

fn repulsion_radius(node: &DslValue) -> f64 {
    if node.get("shape").and_then(semio_framework_value::DslValue::as_str) == Some("rectangle") {
        let (width, height) = (number(node, "width").unwrap_or(40.0), number(node, "height").unwrap_or(40.0));
        return ((width * width + height * height).sqrt() * 0.5).max(8.0);
    }
    number(node, "radius").filter(|radius| *radius > 0.0).unwrap_or(32.0)
}

/// 🗺️ Maps the visible composed board: one body per first-seen node id (committed position as origin, locked placed nodes
/// pinned), one unit spring per distinct node pair whose visible edge endpoints resolve through handles or node ids.
pub fn layout_graph(board: &DslValue) -> WiresLayoutGraph {
    let (mut layout, mut index, mut handles) = (WiresLayoutGraph::default(), HashMap::new(), HashMap::new());
    for node in crate::schema::board_snapshot_nodes(board).iter().filter(|node| visible(node)) {
        let Some(id) = node.get("id").and_then(semio_framework_value::DslValue::as_str) else { continue };
        if index.contains_key(id) {
            continue;
        }
        let at = layout.node_ids.len() as u32;
        index.insert(id.to_string(), at);
        for handle in node.get("handles").and_then(semio_framework_value::DslValue::as_array).unwrap_or(&[]).iter().filter(|handle| visible(handle)) {
            if let Some(handle_id) = handle.get("id").and_then(semio_framework_value::DslValue::as_str) {
                handles.insert(handle_id.to_string(), at);
            }
        }
        let origin = number(node, "x").zip(number(node, "y")).map(|(x, y)| LayoutRunPoint::new(x, y));
        let pinned = origin.is_some() && node.get("locked").and_then(semio_framework_value::DslValue::as_bool) == Some(true);
        layout.graph.nodes.push(LayoutRunNode { entity: layout_run_entity(id), origin, radius: repulsion_radius(node), pinned, anchor: None });
        layout.node_ids.push(id.to_string());
    }
    let resolve = |endpoint: Option<&str>| endpoint.and_then(|endpoint| handles.get(endpoint).or_else(|| index.get(endpoint)).copied());
    let mut seen = BTreeSet::new();
    for edge in crate::schema::board_snapshot_edges(board).iter().filter(|edge| visible(edge)) {
        let (Some(source), Some(target)) = (resolve(edge.get("source").and_then(semio_framework_value::DslValue::as_str)), resolve(edge.get("target").and_then(semio_framework_value::DslValue::as_str))) else { continue };
        if source != target && seen.insert((source.min(target), source.max(target))) {
            layout.graph.edges.push(LayoutRunEdge { source: source.min(target), target: source.max(target), weight: 1.0 });
        }
    }
    layout
}
//#endregion 🔖️Graph

//#region 🔖️Job
/// ✍️ Encodes one layout move as the board child's graph `move-node` op.
pub fn move_encoder(node_ids: Vec<String>) -> impl LayoutRunOpEncoder + 'static {
    move |node: u32, _entity: u64, position: LayoutRunPoint| {
        protocol::OpBinary::encode_op(&SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(node_ids[node as usize].clone()), new_position: SemioPoint2 { x: position.x, y: position.y } })).map_err(|error| LayoutRunEncodeError(format!("{error:?}")))
    }
}

/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional child moves folded in.
fn overlay_positions(layout: &WiresLayoutGraph, provisional: &[Vec<u8>]) -> Vec<LayoutRunPoint> {
    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();
    let moves = provisional.iter().filter_map(|bytes| match <SemioGraphMutation as protocol::OpBinary>::decode_op(bytes).ok()? {
        SemioGraphMutation::MoveNode(payload) => index.get(payload.id.value.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.new_position.x, payload.new_position.y))),
        _ => None,
    });
    layout_run_overlay_positions(&layout.graph, moves)
}

/// 🧵️ Builds the layout run over the composed board the run rests on; a settings change resumes it from the ledger's
/// checkpoint with the provisional child moves folded over the base positions (`📓️wave-W3-F.md` §3.3). A board the layout
/// refuses faults with the named, localized `wires.layout.run`.
pub fn build_job(identity: ToolRunIdentity, board: &DslValue, checkpoint: Option<&[u8]>, provisional: &[Vec<u8>]) -> Result<ToolRunJob, Fault> {
    let layout = layout_graph(board);
    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, provisional), provisional_len: provisional.len() as u32 });
    let job = layout_run_job(identity, &layout.graph, LayoutRunConfig::default(), || move_encoder(layout.node_ids.clone()), resume).map_err(|error| crate::wires_content_fault("wires.layout.run", format!("the board cannot be laid out: {error:?}")))?;
    Ok(Box::new(job))
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
