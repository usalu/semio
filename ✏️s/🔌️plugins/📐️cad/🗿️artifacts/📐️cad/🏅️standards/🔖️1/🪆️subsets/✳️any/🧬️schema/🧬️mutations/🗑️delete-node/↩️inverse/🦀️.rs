//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::DeleteNode;
use crate::mutations::create_node;
use crate::mutations::CadMutation;
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteNode, base: &CadSnapshot) -> Vec<CadMutation> {
    base.nodes.iter().find(|node| node.id == payload.node_id).map(|node| vec![CadMutation::CreateNode(create_node::CreateNode { node: node.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
