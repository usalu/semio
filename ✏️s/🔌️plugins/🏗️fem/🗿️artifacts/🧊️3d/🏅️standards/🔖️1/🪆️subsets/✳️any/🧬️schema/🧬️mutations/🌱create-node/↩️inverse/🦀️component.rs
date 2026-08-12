//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::mutation::CreateNode;
use crate::artifacts::fem3d::mutations::{delete_node, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateNode, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: payload.node.id.clone() })]
}
//#endregion 🔖️Inverse
