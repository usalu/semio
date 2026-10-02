//! 🔺️ Sparse diff builder for `MoveNodes` — every addressed node moves by the payload offset from its BASE position; a
//! node the board lacks is skipped (`mutation.partial`).

use crate::diff::{diff_board_fixture, WiresDiff};
use crate::mutations::{board_node_position, set_node_field, wires_partial, wires_targets_invariant};
use crate::WiresSnapshot;

//#region 🔖️Diff
/// 🏗️ A malformed target list or a non-finite offset is `mutation.invariant`; none left to move is
/// `mutation.target-missing`; a zero offset is `mutation.no-op`; a moved position off the finite board is
/// `mutation.target-mismatch`.
pub fn diff(payload: &super::MoveNodes, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    if let Err(reason) = wires_targets_invariant(&payload.node_ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, payload.node_ids.clone());
    }
    if !payload.dx.is_finite() || !payload.dy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node offset must be finite", payload.node_ids.clone());
    }
    let mut board = crate::wires_working_board(base);
    let placed: Vec<(&String, (f64, f64))> = payload.node_ids.iter().filter_map(|id| board_node_position(&board, id).map(|position| (id, position))).collect();
    if placed.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} node(s) exists", payload.node_ids.len()), payload.node_ids.clone());
    }
    let missing: Vec<String> = payload.node_ids.iter().filter(|id| !placed.iter().any(|(placed_id, _)| placed_id == id)).cloned().collect();
    let messages: Vec<protocol::MutationMessage> = wires_partial(missing, payload.node_ids.len()).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "the drag offset is zero").at(payload.node_ids.clone())]));
    }
    let moved: Vec<(String, f64, f64)> = placed.into_iter().map(|(id, (x, y))| (id.clone(), x + payload.dx, y + payload.dy)).collect();
    if moved.iter().any(|(_, x, y)| !x.is_finite() || !y.is_finite()) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "the moved position leaves the finite board", payload.node_ids.clone());
    }
    for (id, x, y) in moved {
        set_node_field(&mut board, &id, "x", semio_framework_value::ToValue::to_value(&x));
        set_node_field(&mut board, &id, "y", semio_framework_value::ToValue::to_value(&y));
    }
    protocol::MutationOutcome::new(diff_board_fixture(&board)).absorb_messages(messages)
}
//#endregion 🔖️Diff
