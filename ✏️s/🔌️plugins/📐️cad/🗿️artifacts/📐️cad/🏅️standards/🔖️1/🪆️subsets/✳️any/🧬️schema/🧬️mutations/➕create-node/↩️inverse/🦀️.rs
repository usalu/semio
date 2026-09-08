//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::CreateNode;
use crate::mutations::delete_node;
use crate::mutations::CadMutation;
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateNode, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteNode(delete_node::DeleteNode { node_id: payload.node.id.clone() })]
}
//#endregion 🔖️Inverse
