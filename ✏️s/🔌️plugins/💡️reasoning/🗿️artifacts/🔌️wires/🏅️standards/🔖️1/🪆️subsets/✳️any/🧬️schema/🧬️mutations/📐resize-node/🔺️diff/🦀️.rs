//! 🔺️ Sparse diff builder for `ResizeNode` — writes only the extent fields present in the payload.

use crate::artifacts::wires::diff::{diff_board_fixture, WiresDiff};
use crate::artifacts::wires::mutations::set_node_field;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Diff
pub async fn diff(payload: &super::ResizeNode, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    let Some(node) = find_board_node(base, &payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    for value in [payload.new_radius, payload.new_width, payload.new_height].into_iter().flatten() {
        if !value.is_finite() || value <= 0.0 {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" extent must be finite and positive, got {}.", payload.node_id, value), [payload.node_id.clone()]);
        }
    }
    let unchanged = payload.new_radius.map_or(true, |v| node.get("radius").and_then(|value| value.as_f64()) == Some(v))
        && payload.new_width.map_or(true, |v| node.get("width").and_then(|value| value.as_f64()) == Some(v))
        && payload.new_height.map_or(true, |v| node.get("height").and_then(|value| value.as_f64()) == Some(v));
    if unchanged {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" extent is unchanged.", payload.node_id));
    }
    let mut board = crate::artifacts::wires::wires_working_board(base);
    if let Some(radius) = payload.new_radius {
        set_node_field(&mut board, &payload.node_id, "radius", dsl::to_dsl_value(&radius).unwrap_or(DslValue::Null));
    }
    if let Some(width) = payload.new_width {
        set_node_field(&mut board, &payload.node_id, "width", dsl::to_dsl_value(&width).unwrap_or(DslValue::Null));
    }
    if let Some(height) = payload.new_height {
        set_node_field(&mut board, &payload.node_id, "height", dsl::to_dsl_value(&height).unwrap_or(DslValue::Null));
    }
    protocol::MutationOutcome::new(diff_board_fixture(board))
}
//#endregion 🔖️Diff
