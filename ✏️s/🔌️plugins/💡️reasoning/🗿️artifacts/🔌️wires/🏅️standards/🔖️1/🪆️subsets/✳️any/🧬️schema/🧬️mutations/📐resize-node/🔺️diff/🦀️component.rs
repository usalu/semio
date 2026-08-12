//! 🔺️ Sparse diff builder for `ResizeNode` — writes only the extent fields present in the payload.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeNode, base: &WiresSnapshot) -> WiresDiff {
    let mut board = base.board_fixture.clone();
    if let Some(radius) = payload.new_radius {
        set_node_field(&mut board, &payload.node_id, "radius", dsl::to_dsl_value(&radius).unwrap_or(DslValue::Null));
    }
    if let Some(width) = payload.new_width {
        set_node_field(&mut board, &payload.node_id, "width", dsl::to_dsl_value(&width).unwrap_or(DslValue::Null));
    }
    if let Some(height) = payload.new_height {
        set_node_field(&mut board, &payload.node_id, "height", dsl::to_dsl_value(&height).unwrap_or(DslValue::Null));
    }
    diff_board_fixture(board)
}
//#endregion 🔖️Diff
