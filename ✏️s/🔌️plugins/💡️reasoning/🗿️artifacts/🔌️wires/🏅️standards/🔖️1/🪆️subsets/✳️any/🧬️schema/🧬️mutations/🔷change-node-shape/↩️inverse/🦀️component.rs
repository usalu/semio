//! ↩️ Inverse for `ChangeNodeShape` — the OLD `shape` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::wires::engine::find_board_node;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeShape, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("shape")).and_then(|value| value.as_str()) {
        Some(old_shape) => vec![crate::artifacts::wires::mutations::change_node_shape::mutation::change_node_shape(payload.node_id.clone(), old_shape.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
