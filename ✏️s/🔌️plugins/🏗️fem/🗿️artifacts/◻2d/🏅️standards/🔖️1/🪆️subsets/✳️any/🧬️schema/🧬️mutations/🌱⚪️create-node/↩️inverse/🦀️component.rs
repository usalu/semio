//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::mutation::CreateNode;
use crate::artifacts::fem2d::mutations::{delete_node, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateNode, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: payload.node.id.clone() })]
}
//#endregion 🔖️Inverse
