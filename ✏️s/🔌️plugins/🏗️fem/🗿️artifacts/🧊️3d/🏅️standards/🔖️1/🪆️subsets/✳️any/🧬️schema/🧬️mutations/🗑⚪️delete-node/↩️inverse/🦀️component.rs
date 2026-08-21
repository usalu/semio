//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::mutation::DeleteNode;
use crate::artifacts::fem3d::mutations::{create_node, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteNode, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.nodes.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateNode(create_node::mutation::CreateNode { node: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
