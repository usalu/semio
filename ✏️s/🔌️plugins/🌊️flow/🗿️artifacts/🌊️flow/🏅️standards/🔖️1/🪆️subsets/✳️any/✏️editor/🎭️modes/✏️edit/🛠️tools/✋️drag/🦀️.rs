//! ✋️ Flow edit-mode tool — Drag: every node drag commits through the ONE node-drag machine of `🛠️tool-machine`
//! ([`node_drag_commit`], design §13.3). Both hosts journal a released drag as the node-graph gesture record
//! ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset, `nodeGraphEdit`), an agent moves one
//! node (`moveMediaNode`); both leave as ONE `ToolTransaction` of relative `drag-nodes` leaves on the composed `content`
//! child, published as a composed-child transaction (§12). Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §12, §13.3).

use crate::op::FlowMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{Emit, NoConfigMutation};
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord};
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

/// ⏰️ The host clock a drag commit runs on, so a transaction id minted at its upsert is unique per admission AND per
/// moment.
pub fn flow_drag_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🛠️ Commits one release as ONE transaction through the node-drag machine: `prepared` (a wire the same gesture drew), then
/// the `drag-nodes` leaves `records` mean on `base`, the ref minted from the admission's `authoring_seed`, the host clock
/// and `<appId>#<verb>`, the press named by the first record. `None` when nothing lands.
pub fn flow_drag_tool_commit(verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> Option<(protocol::TransactionRef, Vec<SemioFlowMutation>)> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    let leaves: Vec<SemioFlowMutation> = prepared.into_iter().chain(flow_drag_leaves(base, records).into_iter().map(SemioFlowMutation::DragNodes)).collect();
    node_drag_commit(format!("{FLOW_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), gesture, leaves, flow_drag_tool_clock())
}

/// 🧾️ The emit one committed release publishes: ONE composite group whose only member is the `content` child `child_id`,
/// its edit stamped with the ref and labelled from its leaves (design §12). A view without command authority (no authoring
/// seed) publishes the leaves plainly; nothing landed is the empty emit.
pub fn flow_drag_tool_emit(child_id: &str, verb: &str, authoring_seed: &str, base: &SemioFlowSnapshot, prepared: Vec<SemioFlowMutation>, records: &[NodeDragRecord]) -> Emit<FlowMutation, NoConfigMutation> {
    let Some((transaction, leaves)) = flow_drag_tool_commit(verb, authoring_seed, base, prepared, records) else { return Emit::default() };
    let child = ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, &leaves);
    let emit = match authoring_seed.is_empty() {
        true => Emit { child_emits: vec![child], ..Default::default() },
        false => Emit::commit_child_transaction(transaction, vec![child]),
    };
    Emit { ui_scope: UiDirtyScope::Full, ..emit }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
