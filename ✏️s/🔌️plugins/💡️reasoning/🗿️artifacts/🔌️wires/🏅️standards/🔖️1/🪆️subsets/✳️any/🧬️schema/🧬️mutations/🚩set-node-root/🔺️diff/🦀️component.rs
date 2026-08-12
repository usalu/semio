//! 🔺️ Sparse diff builder for `SetNodeRoot`.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetNodeRoot, base: &WiresSnapshot) -> WiresDiff {
    let mut board = base.board_fixture.clone();
    set_node_field(&mut board, &payload.node_id, "root", DslValue::Bool(payload.new_root));
    diff_board_fixture(board)
}
//#endregion 🔖️Diff
