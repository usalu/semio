//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::CreateNode;
use crate::standards::v1::subsets::any::schema::mutations::delete_node;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_node::DeleteNode>>(payload: &CreateNode, _base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_node::DeleteNode { id: payload.node.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
