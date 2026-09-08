//! ↩️ Inverse for `DeleteNode` — recreates the removed node from its full captured BASE payload.
//! Missing target ⇒ `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteNode, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id) {
        Some(node) => vec![crate::mutations::create_node::create_node(node)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
