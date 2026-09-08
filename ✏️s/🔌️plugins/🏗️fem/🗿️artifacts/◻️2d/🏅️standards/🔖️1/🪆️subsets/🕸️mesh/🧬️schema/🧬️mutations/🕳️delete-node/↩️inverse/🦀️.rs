//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::DeleteNode;
use crate::standards::v1::subsets::any::schema::mutations::{create_node, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteNode, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.nodes.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateNode(create_node::CreateNode { node: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
