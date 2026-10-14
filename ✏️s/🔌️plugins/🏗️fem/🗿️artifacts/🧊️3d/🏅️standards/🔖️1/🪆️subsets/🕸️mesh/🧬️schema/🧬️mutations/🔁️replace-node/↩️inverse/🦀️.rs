//! ↩️ Inverse for `ReplaceNode` — recovers the pre-mutation node from `base`.
use super::ReplaceNode;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<replace_node::ReplaceNode>>(payload: &ReplaceNode, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.nodes.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceNode { id: payload.id.clone(), new_node: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
