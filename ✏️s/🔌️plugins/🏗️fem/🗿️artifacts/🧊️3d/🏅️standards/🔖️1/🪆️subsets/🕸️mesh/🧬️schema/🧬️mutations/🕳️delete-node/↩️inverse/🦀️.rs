//! ↩️ Inverse for `DeleteNode` — recreates the captured node from `base`.
use super::DeleteNode;
use crate::standards::v1::subsets::any::schema::mutations::create_node;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_node::CreateNode>>(payload: &DeleteNode, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.nodes.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(create_node::CreateNode { node: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
