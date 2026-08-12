//! ↩️ `create-node` — undo is `delete-node`, unless `base` already had this id (then `create` was
//! a no-op and there's nothing to undo).

use crate::artifacts::mathematical::mutations::delete_node;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

use super::mutation::CreateNode;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateNode, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    if base.graph.nodes.iter().any(|node| node.id == payload.id) {
        return Vec::new();
    }
    vec![MathematicalMutation::DeleteNode(delete_node::mutation::DeleteNode { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
