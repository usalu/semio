//! 🔺️ Sparse diff builder for `ConnectNodes` — delegates to the schema diff facet's own
//! `fixtures_after_add_edge` (a real targeted board+wires-fixture rebuild).
use crate::artifacts::wires::diff::{diff_wires_and_board, fixtures_after_add_edge, WiresDiff};
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectNodes, base: &WiresSnapshot) -> WiresDiff {
    let (wires, board) = fixtures_after_add_edge(base, &payload.edge, &payload.relationship);
    diff_wires_and_board(wires, board)
}
//#endregion 🔖️Diff
