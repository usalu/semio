//! 🔺️ Sparse diff builder for `ChangeNodeShape`.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeShape, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let Some(node) = find_board_node(base, &payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if node.get("shape").and_then(|value| value.as_str()) == Some(payload.new_shape.as_str()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" shape is already \"{}\".", payload.node_id, payload.new_shape));
    }
    let mut board = crate::artifacts::wires::wires_working_board(base);
    set_node_field(&mut board, &payload.node_id, "shape", DslValue::String(payload.new_shape.clone()));
    protocol::MutationOutcome::new(diff_board_fixture(board))
}
//#endregion 🔖️Diff
