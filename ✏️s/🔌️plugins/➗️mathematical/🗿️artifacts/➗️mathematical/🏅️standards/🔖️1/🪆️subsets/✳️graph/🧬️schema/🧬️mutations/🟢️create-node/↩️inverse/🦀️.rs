//! ↩️ `create-node` — undo is `delete-node`, unless `base` already had this id (then `create` was
//! a no-op and there's nothing to undo).

use crate::artifacts::mathematical::standards::v1::subsets::graph::schema::mutations::delete_node;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateNode, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    if crate::artifacts::mathematical::mathematical_graph(base).nodes.iter().any(|node| node.id == payload.id) {
        return Vec::new();
    }
    vec![MathematicalMutation::DeleteNode(delete_node::DeleteNode { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
