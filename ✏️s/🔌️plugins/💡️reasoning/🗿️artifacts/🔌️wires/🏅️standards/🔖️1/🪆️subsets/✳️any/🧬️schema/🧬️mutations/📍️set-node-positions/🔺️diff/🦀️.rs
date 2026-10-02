//! 🔺️ Sparse diff builder for `SetNodePositions` — every addressed node lands at its payload position; a node the board
//! lacks is skipped (`mutation.partial`).

use crate::diff::{diff_board_fixture, WiresDiff};
use crate::mutations::{board_node_position, set_node_field, wires_partial, wires_targets_invariant};
use crate::WiresSnapshot;

//#region 🔖️Diff
/// 🏗️ A malformed position list is `mutation.invariant`; none left to place is `mutation.target-missing`; every node
/// already there is `mutation.no-op`.
pub fn diff(payload: &super::SetNodePositions, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let node_ids = payload.node_ids();
    if let Err(reason) = wires_targets_invariant(&node_ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, node_ids);
    }
    if payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node position must be finite", node_ids);
    }
    let mut board = crate::wires_working_board(base);
    let placed: Vec<(&super::WiresNodePosition, (f64, f64))> = payload.positions.iter().filter_map(|position| board_node_position(&board, &position.node_id).map(|current| (position, current))).collect();
    if placed.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} node(s) exists", node_ids.len()), node_ids);
    }
    let missing: Vec<String> = node_ids.iter().filter(|id| !placed.iter().any(|(position, _)| &position.node_id == *id)).cloned().collect();
    let messages: Vec<protocol::MutationMessage> = wires_partial(missing, node_ids.len()).into_iter().collect();
    if placed.iter().all(|(position, current)| (position.x, position.y) == *current) {
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "every node already sits at its position").at(node_ids)]));
    }
    for (position, _) in placed {
        set_node_field(&mut board, &position.node_id, "x", semio_framework_value::ToValue::to_value(&position.x));
        set_node_field(&mut board, &position.node_id, "y", semio_framework_value::ToValue::to_value(&position.y));
    }
    protocol::MutationOutcome::new(diff_board_fixture(&board)).absorb_messages(messages)
}
//#endregion 🔖️Diff
