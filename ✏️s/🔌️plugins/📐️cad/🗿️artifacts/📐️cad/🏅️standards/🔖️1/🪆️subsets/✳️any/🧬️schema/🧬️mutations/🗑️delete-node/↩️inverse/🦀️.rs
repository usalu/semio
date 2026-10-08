//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::DeleteNode;
use crate::mutations::create_node;
use crate::mutations::CadMutation;
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteNode, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok(base.nodes.iter().enumerate().find(|(_, node)| node.id == payload.node_id).map(|(index, node)| vec![CadMutation::CreateNode(create_node::CreateNode { node: node.clone(), index: u32::try_from(index).ok() })]).unwrap_or_default())
}
//#endregion 🔖️Inverse
