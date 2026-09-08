//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::CreateNode;
use crate::standards::v1::subsets::any::schema::mutations::{delete_node, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateNode, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: payload.node.id.clone() })]
}
//#endregion 🔖️Inverse
