//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the created id.
use super::CreateNode;
use crate::standards::v1::subsets::any::schema::mutations::delete_node;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode>>(payload: &CreateNode, _base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id: payload.node.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
