//! ↩️ Inverse for `CreateSolid` — always a `delete-solid` of the created id.
use super::CreateSolid;
use crate::standards::v1::subsets::any::schema::mutations::delete_solid;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_solid::DeleteSolid>>(payload: &CreateSolid, _base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_solid::DeleteSolid { id: payload.solid.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
