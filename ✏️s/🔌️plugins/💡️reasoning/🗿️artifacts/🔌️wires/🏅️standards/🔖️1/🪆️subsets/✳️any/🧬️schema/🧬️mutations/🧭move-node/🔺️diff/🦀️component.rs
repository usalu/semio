//! 🔺️ Sparse diff builder for `MoveNode` — targeted `x`/`y` field writes on the addressed node via
//! the dispatch facet's shared `set_node_field`, never a whole-fixture recompute.
use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveNode, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let Some(node) = find_board_node(base, &payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.node_id, payload.new_x, payload.new_y), [payload.node_id.clone()]);
    }
    let (x, y) = crate::artifacts::wires::schema::node_position(&node);
    if x == payload.new_x && y == payload.new_y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.node_id, payload.new_x, payload.new_y));
    }
    let mut board = crate::artifacts::wires::wires_working_board(base);
    set_node_field(&mut board, &payload.node_id, "x", dsl::to_dsl_value(&payload.new_x).unwrap_or(DslValue::Null));
    set_node_field(&mut board, &payload.node_id, "y", dsl::to_dsl_value(&payload.new_y).unwrap_or(DslValue::Null));
    protocol::MutationOutcome::new(diff_board_fixture(board))
}
//#endregion 🔖️Diff
