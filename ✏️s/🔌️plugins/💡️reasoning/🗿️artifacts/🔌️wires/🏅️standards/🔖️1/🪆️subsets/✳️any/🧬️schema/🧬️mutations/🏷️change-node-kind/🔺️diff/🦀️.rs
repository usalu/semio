//! 🔺️ Sparse diff builder for `ChangeNodeKind`.

use crate::diff::{diff_board_fixture, WiresDiff};
use crate::mutations::set_node_field;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKind, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let Some(node) = find_board_node(base, &payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if node.get("nodeKind").and_then(|value| value.as_str()) == Some(payload.new_node_kind.as_str()) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" kind is already \"{}\".", payload.node_id, payload.new_node_kind));
    }
    let mut board = crate::wires_working_board(base);
    set_node_field(&mut board, &payload.node_id, "nodeKind", DslValue::String(payload.new_node_kind.clone()));
    protocol::MutationOutcome::new(diff_board_fixture(&board))
}
//#endregion 🔖️Diff
