//! ↩️ `create-node` — undo is `delete-node`, unless `base` already had this id (then `create` was
//! a no-op and there's nothing to undo).

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::delete_node;
use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateNode, base: &EquationSnapshot) -> Vec<EquationMutation> {
    if crate::artifacts::equation::equation_graph(base).nodes.iter().any(|node| node.id == payload.id) {
        return Vec::new();
    }
    vec![EquationMutation::DeleteNode(delete_node::DeleteNode { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
