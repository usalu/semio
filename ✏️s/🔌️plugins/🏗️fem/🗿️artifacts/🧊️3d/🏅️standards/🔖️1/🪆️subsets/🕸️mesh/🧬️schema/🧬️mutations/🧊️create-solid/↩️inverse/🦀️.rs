//! ↩️ Inverse for `CreateSolid` — always a `delete-solid` of the created id.
use super::CreateSolid;
use crate::standards::v1::subsets::any::schema::mutations::{delete_solid,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSolid, _base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Fem3dMutation::DeleteSolid(delete_solid::DeleteSolid { id: payload.solid.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
