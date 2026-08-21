//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::mutation::DeleteNode;
use crate::artifacts::cad::mutations::create_node;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteNode, base: &CadSnapshot) -> Vec<CadMutation> {
    base.nodes.iter().find(|node| node.id == payload.node_id).map(|node| vec![CadMutation::CreateNode(create_node::mutation::CreateNode { node: node.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
