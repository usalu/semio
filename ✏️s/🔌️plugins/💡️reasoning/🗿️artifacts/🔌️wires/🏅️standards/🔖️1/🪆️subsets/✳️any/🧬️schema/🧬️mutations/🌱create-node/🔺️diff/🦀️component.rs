//! 🔺️ Sparse diff builder for `CreateNode` — delegates to the schema diff facet's own
//! `board_after_add_node` (a real targeted board-fixture rebuild, never apply-then-capture).
use crate::artifacts::wires::diff::{board_after_add_node, diff_board_fixture, WiresDiff};
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, base: &WiresSnapshot) -> WiresDiff {
    diff_board_fixture(board_after_add_node(base, &payload.node))
}
//#endregion 🔖️Diff
