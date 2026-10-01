//! ✋️ WFC 3D edit-mode tool — Drag: every node drag on the shared `wfc-graph` canvas commits through the ONE
//! node-drag machine of `🛠️tool-machine` ([`node_drag_commit`], design §13.3). A host journals a released drag as the
//! node-graph gesture record ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset in
//! canvas units); a host that hands back its whole graph instead (`setHostSnapshot`) is read as the records its
//! displaced nodes make. Either way the release leaves as ONE `ToolTransaction` of relative `drag-slots` leaves
//! (plus the wires the gesture drew or cut), one edit, one history row. The canvas is a plan, so a drag never moves a
//! slot along `z`. Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §13.3).

use crate::editor::wfc3d::config::Wfc3dConfigMutation;
use crate::mutations::{drag_slots, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord};

/// 🪪️ The editor app id every drag-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const WFC_3D_EDITOR_APP_ID: &str = "s.wfc.wfc3d@1/*#editor";

/// 🧮️ The `drag-slots` leaves `records` mean on `base`, one per record that moves, in record order, each over the
/// record's nodes the document holds and its canvas offset divided by `unit` (canvas units per document unit), with
/// `dz = 0`. A record naming no slot of this document, or one whose offset moves nothing, yields no leaf.
pub fn wfc3d_drag_leaves(base: &Wfc3dSnapshot, records: &[NodeDragRecord], unit: f64) -> Vec<Wfc3dMutation> {
    records
        .iter()
        .filter(|record| record.moves() && unit.is_finite() && unit > 0.0)
        .filter_map(|record| {
            let targets: Vec<String> = record.node_ids.iter().filter(|id| base.slots.iter().any(|slot| &slot.id == *id)).cloned().collect();
            (!targets.is_empty()).then(|| drag_slots(targets, record.dx / unit, record.dy / unit, 0.0))
        })
        .collect()
}

/// ⏰️ The host clock a drag commit runs on, so a transaction id minted at its upsert is unique per admission AND per
/// moment.
pub fn wfc3d_drag_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🛠️ Commits one release as ONE transaction through the node-drag machine: `prepared` (the wires the same gesture
/// drew or cut), then the `drag-slots` leaves `records` mean on `base`, the ref minted from the admission's
/// `authoring_seed`, the host clock and `<appId>#<verb>`, the press named by the first record. `None` when nothing
/// lands.
pub fn wfc3d_drag_tool_commit(verb: &str, authoring_seed: &str, base: &Wfc3dSnapshot, prepared: Vec<Wfc3dMutation>, records: &[NodeDragRecord], unit: f64) -> Option<(protocol::TransactionRef, Vec<Wfc3dMutation>)> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    let leaves: Vec<Wfc3dMutation> = prepared.into_iter().chain(wfc3d_drag_leaves(base, records, unit)).collect();
    node_drag_commit(format!("{WFC_3D_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), gesture, leaves, wfc3d_drag_tool_clock())
}

/// 🧾️ The emit one committed release publishes: ONE edit stamped with the ref. A view without command authority (no
/// authoring seed: a render or test view) publishes the leaves plainly; nothing landed is the empty emit.
pub fn wfc3d_drag_tool_emit(verb: &str, authoring_seed: &str, base: &Wfc3dSnapshot, prepared: Vec<Wfc3dMutation>, records: &[NodeDragRecord], unit: f64, description: String) -> Emit<Wfc3dMutation, Wfc3dConfigMutation> {
    let Some((transaction, leaves)) = wfc3d_drag_tool_commit(verb, authoring_seed, base, prepared, records, unit) else { return Emit::default() };
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
