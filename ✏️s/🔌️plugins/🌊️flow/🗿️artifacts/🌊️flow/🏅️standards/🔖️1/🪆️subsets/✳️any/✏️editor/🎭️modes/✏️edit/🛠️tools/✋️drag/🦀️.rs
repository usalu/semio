//! ✋️ Flow edit-mode tool — Drag: every node drag commits through the ONE node-drag machine of `🛠️tool-machine`
//! ([`node_drag_emit`], design §13.3). Both hosts journal a released drag as the node-graph gesture record
//! ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset, `nodeGraphEdit`), an agent moves one
//! node (`moveMediaNode`); both leave as ONE `ToolTransaction` of relative `drag-nodes` leaves on the composed `content`
//! child, published as a composed-child transaction (§12). Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §12, §13.3).

use crate::FlowMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{Emit, NoConfigMutation};
use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeDragRecord};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{drag_nodes::DragNodes, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// 🪪️ The editor app id every drag-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const FLOW_EDITOR_APP_ID: &str = "s.flow.flow@1/*#editor";

/// 🧮️ The `drag-nodes` leaves `records` mean on `base`, one per record that moves, in record order, each over the record's
/// nodes the content holds — a record naming no node of this flow, or one whose offset moves nothing, yields no leaf.
pub fn flow_drag_leaves(base: &SemioFlowSnapshot, records: &[NodeDragRecord]) -> Vec<DragNodes> {
    records
        .iter()
        .filter(|record| record.moves())
        .filter_map(|record| {
            let targets: Vec<String> = record.node_ids.iter().filter(|id| base.nodes.iter().any(|node| node.id == **id)).cloned().collect();
            (!targets.is_empty()).then(|| DragNodes { targets, dx: record.dx, dy: record.dy })
        })
        .collect()
}

/// 🛠️ One release through the node-drag machine: `prepared` (a wire the same gesture drew), then the `drag-nodes` leaves
/// `records` mean on `base`, as the tool `<appId>#<verb>` of the press the first record names — ONE committed
/// transaction, the leaves plainly for a view without an authoring seed, or nothing when nothing lands.
pub fn flow_drag_tool(verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> NodeDragEmit<SemioFlowMutation> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    node_drag_emit(FLOW_EDITOR_APP_ID, verb, authoring_seed, gesture, prepared.into_iter().chain(flow_drag_leaves(base, records).into_iter().map(SemioFlowMutation::DragNodes)).collect())
}

/// 🧾️ The emit one release publishes on the `content` child `child_id` ([`Emit::node_drag_child`], design §12): ONE
/// composite group whose member edit carries the transaction and is labelled from its leaves, the leaves plainly for a
/// view without command authority, or the empty emit when nothing landed.
pub fn flow_drag_tool_emit(child_id: &str, verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> Emit<FlowMutation, NoConfigMutation> {
    match flow_drag_tool(verb, authoring_seed, base, prepared, records) {
        NodeDragEmit::Nothing => Emit::default(),
        drag => Emit { ui_scope: UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", child_id) },
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
