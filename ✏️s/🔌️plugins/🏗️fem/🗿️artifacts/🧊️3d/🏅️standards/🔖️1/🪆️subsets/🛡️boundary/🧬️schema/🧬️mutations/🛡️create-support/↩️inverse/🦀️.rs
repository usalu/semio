//! ↩️ Inverse for `CreateSupport` — always a `delete-support` of the created id.
use super::CreateSupport;
use crate::standards::v1::subsets::any::schema::mutations::delete_support;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_support::DeleteSupport>>(payload: &CreateSupport, _base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_support::DeleteSupport { id: payload.support.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
