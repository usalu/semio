//! ↩️ Inverse for `ChangeNodeShape` — the OLD `shape` looked up from BASE. Missing target ⇒
//! `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeNodeShape, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("shape").and_then(|value| value.as_str()).map(str::to_string)) {
        Some(old_shape) => vec![crate::mutations::change_node_shape::change_node_shape(payload.node_id.clone(), old_shape)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
