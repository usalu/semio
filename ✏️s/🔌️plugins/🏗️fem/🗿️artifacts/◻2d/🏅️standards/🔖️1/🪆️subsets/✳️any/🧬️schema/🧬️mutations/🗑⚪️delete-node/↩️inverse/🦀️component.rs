//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::mutation::DeleteNode;
use crate::artifacts::fem2d::mutations::{create_node, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteNode, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.nodes
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem2dMutation::CreateNode(create_node::mutation::CreateNode { node: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
