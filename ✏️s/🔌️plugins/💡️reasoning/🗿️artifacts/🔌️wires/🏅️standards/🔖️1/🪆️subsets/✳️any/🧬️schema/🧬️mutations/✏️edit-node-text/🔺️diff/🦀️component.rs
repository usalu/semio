//! 🔺️ Sparse diff builder for `EditNodeText`.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditNodeText, base: &WiresSnapshot) -> WiresDiff {
    let mut board = base.board_fixture.clone();
    set_node_field(&mut board, &payload.node_id, "text", DslValue::String(payload.new_text.clone()));
    diff_board_fixture(board)
}
//#endregion 🔖️Diff
