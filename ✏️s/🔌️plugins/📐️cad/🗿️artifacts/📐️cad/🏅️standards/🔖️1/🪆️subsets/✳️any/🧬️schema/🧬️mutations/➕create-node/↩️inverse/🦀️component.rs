//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::mutation::CreateNode;
use crate::artifacts::cad::mutations::delete_node;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateNode, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteNode(delete_node::mutation::DeleteNode { node_id: payload.node.id.clone() })]
}
//#endregion 🔖️Inverse
