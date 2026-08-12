//! 🔺️ Sparse diff builder for `DisconnectNodes` — delegates to the schema diff facet's own
//! `fixtures_after_remove_edge`.
use crate::artifacts::wires::diff::{diff_wires_and_board, fixtures_after_remove_edge, WiresDiff};
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectNodes, base: &WiresSnapshot) -> WiresDiff {
    let (wires, board) = fixtures_after_remove_edge(base, &payload.edge_id);
    diff_wires_and_board(wires, board)
}
//#endregion 🔖️Diff
