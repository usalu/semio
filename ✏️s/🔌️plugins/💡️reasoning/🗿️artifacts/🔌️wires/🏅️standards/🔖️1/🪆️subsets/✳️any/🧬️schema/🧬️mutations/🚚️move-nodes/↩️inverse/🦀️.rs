//! ↩️ Inverse for `MoveNodes` — ONE absolute `set-node-positions` row putting every moved node back at its BASE position
//! (never a negated offset, which floating point cannot undo exactly), so a multi-node drag stays one point-invertible row.

use crate::mutations::{board_node_position, set_node_positions, wires_targets_invariant, WiresMutation, WiresNodePosition};
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveNodes, base: &WiresSnapshot) -> Vec<WiresMutation> {
    if wires_targets_invariant(&payload.node_ids).is_err() || !payload.dx.is_finite() || !payload.dy.is_finite() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let board = crate::wires_working_board(base);
    let positions: Vec<WiresNodePosition> = payload.node_ids.iter().filter_map(|id| board_node_position(&board, id).map(|(x, y)| WiresNodePosition { node_id: id.clone(), x, y })).collect();
    if positions.is_empty() {
        return Vec::new();
    }
    vec![set_node_positions(positions)]
}
//#endregion 🔖️Inverse
