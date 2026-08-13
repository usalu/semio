//! 🔺️ Sparse diff builder for `ChangeNodeKind`.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeKind, base: &WiresSnapshot) -> WiresDiff {
    let mut board = crate::artifacts::wires::wires_working_board(base);
    set_node_field(&mut board, &payload.node_id, "nodeKind", DslValue::String(payload.new_node_kind.clone()));
    diff_board_fixture(board)
}
//#endregion 🔖️Diff
