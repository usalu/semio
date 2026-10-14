//! ↩️ Inverse for `RenameNode` — recovers the pre-mutation `label` from `base`.
use super::RenameNode;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_node::RenameNode>>(payload: &RenameNode, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.nodes.iter().find(|node| node.id == payload.node_id).map(|node| vec![Op::from(RenameNode { node_id: payload.node_id.clone(), new_label: node.label.clone() })]).unwrap_or_default())
}
//#endregion 🔖️Inverse
