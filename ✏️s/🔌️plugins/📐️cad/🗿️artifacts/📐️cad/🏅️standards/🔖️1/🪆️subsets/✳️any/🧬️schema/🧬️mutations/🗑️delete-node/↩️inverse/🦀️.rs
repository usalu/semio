//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::DeleteNode;
use crate::create_node;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_node::CreateNode>>(payload: &DeleteNode, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.nodes.iter().enumerate().find(|(_, node)| node.id == payload.node_id).map(|(index, node)| vec![Op::from(create_node::CreateNode { node: node.clone(), index: u32::try_from(index).ok() })]).unwrap_or_default())
}
//#endregion 🔖️Inverse
