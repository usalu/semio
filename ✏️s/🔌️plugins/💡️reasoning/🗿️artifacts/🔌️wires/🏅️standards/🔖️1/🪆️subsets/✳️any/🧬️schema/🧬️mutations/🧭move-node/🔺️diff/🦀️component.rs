//! 🔺️ Sparse diff builder for `MoveNode` — targeted `x`/`y` field writes on the addressed node via
//! the dispatch facet's shared `set_node_field`, never a whole-fixture recompute.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveNode, base: &WiresSnapshot) -> WiresDiff {
    let mut board = base.board_fixture.clone();
    set_node_field(&mut board, &payload.node_id, "x", dsl::to_dsl_value(&payload.new_x).unwrap_or(DslValue::Null));
    set_node_field(&mut board, &payload.node_id, "y", dsl::to_dsl_value(&payload.new_y).unwrap_or(DslValue::Null));
    diff_board_fixture(board)
}
//#endregion 🔖️Diff
