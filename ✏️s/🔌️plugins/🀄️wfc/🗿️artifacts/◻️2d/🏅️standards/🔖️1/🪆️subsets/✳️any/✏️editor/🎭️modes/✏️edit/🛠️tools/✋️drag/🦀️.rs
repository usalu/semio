//! ✋️ WFC 2D edit-mode tool — Drag: every node drag on the `wfc-graph` canvas commits through the ONE node-drag machine
//! of `🛠️tool-machine` ([`node_drag_commit`], design §13.3). A host journals a released drag as the node-graph gesture
//! record ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset in canvas units); a host
//! that hands back its whole graph instead (`setHostSnapshot`) is read as the records its displaced nodes make. Either
//! way the release leaves as ONE `ToolTransaction` of relative `drag-slots` leaves (plus the wire a gesture drew),
//! one edit, one history row. Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §13.3).

use crate::editor::wfc2d::config::Wfc2dConfigMutation;
use crate::mutations::{drag_slots, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord};

/// 🪪️ The editor app id every drag-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const WFC_2D_EDITOR_APP_ID: &str = "s.wfc.wfc2d@1/*#editor";

/// 🧮️ The `drag-slots` leaves `records` mean on `base`, one per record that moves, in record order, each over the
/// record's nodes the document holds and its canvas offset divided by `scale` (canvas units per document unit). A
/// record naming no slot of this document, or one whose offset moves nothing, yields no leaf.
pub fn wfc2d_drag_leaves(base: &Wfc2dSnapshot, records: &[NodeDragRecord], scale: f64) -> Vec<Wfc2dMutation> {
    records
        .iter()
        .filter(|record| record.moves() && scale.is_finite() && scale > 0.0)
        .filter_map(|record| {
            let targets: Vec<String> = record.node_ids.iter().filter(|id| base.slots.iter().any(|slot| &slot.id == *id)).cloned().collect();
            (!targets.is_empty()).then(|| drag_slots(targets, record.dx / scale, record.dy / scale))
        })
        .collect()
}

/// ⏰️ The host clock a drag commit runs on, so a transaction id minted at its upsert is unique per admission AND per
/// moment.
pub fn wfc2d_drag_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🛠️ Commits one release as ONE transaction through the node-drag machine: `prepared` (a wire the same gesture drew
/// or cut), then the `drag-slots` leaves `records` mean on `base`, the ref minted from the admission's
/// `authoring_seed`, the host clock and `<appId>#<verb>`, the press named by the first record. `None` when nothing
/// lands.
pub fn wfc2d_drag_tool_commit(verb: &str, authoring_seed: &str, base: &Wfc2dSnapshot, prepared: Vec<Wfc2dMutation>, records: &[NodeDragRecord], scale: f64) -> Option<(protocol::TransactionRef, Vec<Wfc2dMutation>)> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    let leaves: Vec<Wfc2dMutation> = prepared.into_iter().chain(wfc2d_drag_leaves(base, records, scale)).collect();
    node_drag_commit(format!("{WFC_2D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), gesture, leaves, wfc2d_drag_tool_clock())
}

/// 🧾️ The emit one committed release publishes: ONE edit stamped with the ref. A view without command authority (no
/// authoring seed: a render or test view) publishes the leaves plainly; nothing landed is the empty emit.
pub fn wfc2d_drag_tool_emit(verb: &str, authoring_seed: &str, base: &Wfc2dSnapshot, prepared: Vec<Wfc2dMutation>, records: &[NodeDragRecord], scale: f64, description: String) -> Emit<Wfc2dMutation, Wfc2dConfigMutation> {
    let Some((transaction, leaves)) = wfc2d_drag_tool_commit(verb, authoring_seed, base, prepared, records, scale) else { return Emit::default() };
    let emit = match authoring_seed.is_empty() {
        true => Emit { artifact_mutations: leaves, ..Default::default() },
        false => Emit::commit_transaction(transaction, leaves),
    };
    Emit { description: Some(description), ..emit }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
