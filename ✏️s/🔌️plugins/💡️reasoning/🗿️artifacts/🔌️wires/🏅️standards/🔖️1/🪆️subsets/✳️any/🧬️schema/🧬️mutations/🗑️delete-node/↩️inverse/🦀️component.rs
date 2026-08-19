//! ↩️ Inverse for `DeleteNode` — recreates the removed node from its full captured BASE payload.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteNode, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id) {
        Some(node) => vec![crate::artifacts::wires::mutations::create_node::mutation::create_node(node.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
