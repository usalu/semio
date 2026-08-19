//! ↩️ Inverse for `ChangeNodeShape` — the OLD `shape` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeNodeShape, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("shape").and_then(|value| value.as_str()).map(str::to_string)) {
        Some(old_shape) => vec![crate::artifacts::wires::mutations::change_node_shape::mutation::change_node_shape(payload.node_id.clone(), old_shape)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
