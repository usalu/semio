//! ✋️ WFC 3D edit-mode tool — Drag: every node drag on the shared `wfc-graph` canvas commits through the ONE
//! node-drag machine of `🛠️tool-machine` ([`node_drag_emit`], design §13.3). A host journals a released drag as the
//! node-graph gesture record ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset in
//! canvas units), decoded by the shared `node_graph_edit_rows`. The release leaves as ONE `ToolTransaction` of relative
//! `drag-slots` leaves (plus the wire and slot edits the gesture drew, cut or deleted), one edit, one history row. The canvas is a plan, so a drag never moves a
//! slot along `z`. Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §13.3).

use crate::mutations::{drag_slots, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;
use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeDragRecord};

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

/// 🛠️ One release through the node-drag machine: `prepared` (the wires the same gesture drew or cut), then the
/// `drag-slots` leaves `records` mean on `base`, as the tool `<appId>#<verb>` of the press the first record names —
/// ONE committed transaction, the leaves plainly for a view without an authoring seed, or nothing when nothing lands.
pub fn wfc3d_drag_tool(verb: &str, authoring_seed: &str, base: &Wfc3dSnapshot, prepared: Vec<Wfc3dMutation>, records: &[NodeDragRecord], unit: f64) -> NodeDragEmit<Wfc3dMutation> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    node_drag_emit(WFC_3D_EDITOR_APP_ID, verb, authoring_seed, gesture, prepared.into_iter().chain(wfc3d_drag_leaves(base, records, unit)).collect())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
