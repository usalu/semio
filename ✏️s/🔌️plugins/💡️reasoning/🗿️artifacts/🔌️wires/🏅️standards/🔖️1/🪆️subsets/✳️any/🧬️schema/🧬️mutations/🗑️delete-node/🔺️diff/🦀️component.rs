//! 🔺️ Sparse diff builder for `DeleteNode` — delegates to the schema diff facet's own
//! `board_after_remove_node` (a targeted `retain`, never apply-then-capture).
use crate::artifacts::wires::diff::{board_after_remove_node, diff_board_fixture, WiresDiff};
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &WiresSnapshot) -> WiresDiff {
    diff_board_fixture(board_after_remove_node(base, &payload.node_id))
}
//#endregion 🔖️Diff
