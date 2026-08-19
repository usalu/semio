//! 🔺️ Sparse diff builder for `EditNodeText`.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::EditNodeText, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let Some(node) = find_board_node(base, &payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if node.get("text").and_then(|value| value.as_str()) == Some(payload.new_text.as_str()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" text is unchanged.", payload.node_id));
    }
    let mut board = crate::artifacts::wires::wires_working_board(base);
    set_node_field(&mut board, &payload.node_id, "text", DslValue::String(payload.new_text.clone()));
    protocol::MutationOutcome::new(diff_board_fixture(board))
}
//#endregion 🔖️Diff
