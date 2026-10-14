//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::CreateNode;
use crate::delete_node;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_node::DeleteNode>>(payload: &CreateNode, _base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(vec![Op::from(delete_node::DeleteNode { node_id: payload.node.id.clone() })])
}
//#endregion 🔖️Inverse
