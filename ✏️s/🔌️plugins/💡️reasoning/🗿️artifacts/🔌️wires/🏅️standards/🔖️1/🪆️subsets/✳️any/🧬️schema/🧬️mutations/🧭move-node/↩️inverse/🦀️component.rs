//! ↩️ Inverse for `MoveNode` — the OLD position looked up from BASE, never a captured offset.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::schema::node_position;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::MoveNode, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id) {
        Some(node) => {
            let (old_x, old_y) = node_position(&node);
            vec![crate::artifacts::wires::mutations::move_node::mutation::move_node(payload.node_id.clone(), old_x, old_y)]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
