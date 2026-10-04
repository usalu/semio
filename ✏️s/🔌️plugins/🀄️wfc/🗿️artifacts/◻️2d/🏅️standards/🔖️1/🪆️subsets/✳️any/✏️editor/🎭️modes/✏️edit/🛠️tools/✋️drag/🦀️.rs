//! ✋️ WFC 2D edit-mode tool — Drag: every node drag on the `wfc-graph` canvas commits through the ONE node-drag machine
//! of `🛠️tool-machine` ([`node_drag_emit`], design §13.3). A host journals a released drag as the node-graph gesture
//! record ([`NodeDragRecord`]: the press, the moved node ids and their ONE relative offset in canvas units), decoded by
//! the shared `node_graph_edit_rows`. The release leaves as ONE `ToolTransaction` of relative `drag-slots` leaves (plus
//! the wire and slot edits the gesture drew, cut or deleted),
//! one edit, one history row. Tool state is never history; the yielded leaves are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §13.3).

use crate::mutations::{drag_slots, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;
use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeDragRecord};

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

/// 🛠️ One release through the node-drag machine: `prepared` (the wires the same gesture drew or cut), then the
/// `drag-slots` leaves `records` mean on `base`, as the tool `<appId>#<verb>` of the press the first record names —
/// ONE committed transaction, the leaves plainly for a view without an authoring seed, or nothing when nothing lands.
pub fn wfc2d_drag_tool(verb: &str, authoring_seed: &str, base: &Wfc2dSnapshot, prepared: Vec<Wfc2dMutation>, records: &[NodeDragRecord], scale: f64) -> NodeDragEmit<Wfc2dMutation> {
    let gesture = records.first().map_or(verb, |record| record.gesture_id.as_str());
    node_drag_emit(WFC_2D_EDITOR_APP_ID, verb, authoring_seed, gesture, prepared.into_iter().chain(wfc2d_drag_leaves(base, records, scale)).collect())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
