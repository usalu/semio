//! ↩️ Inverse for `SetNodePositions` — ONE `set-node-positions` row with every placed node's BASE position; a payload that
//! moves nothing has no inverse.

use crate::mutations::{board_node_position, set_node_positions, wires_targets_invariant, WiresMutation, WiresNodePosition};
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::SetNodePositions, base: &WiresSnapshot) -> Vec<WiresMutation> {
    if wires_targets_invariant(&payload.node_ids()).is_err() || payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return Vec::new();
    }
    let board = crate::wires_working_board(base);
    let placed: Vec<(WiresNodePosition, bool)> = payload
        .positions
        .iter()
        .filter_map(|position| board_node_position(&board, &position.node_id).map(|(x, y)| (WiresNodePosition { node_id: position.node_id.clone(), x, y }, (x, y) != (position.x, position.y))))
        .collect();
    if !placed.iter().any(|(_, moves)| *moves) {
        return Vec::new();
    }
    vec![set_node_positions(placed.into_iter().map(|(position, _)| position).collect())]
}
//#endregion 🔖️Inverse
