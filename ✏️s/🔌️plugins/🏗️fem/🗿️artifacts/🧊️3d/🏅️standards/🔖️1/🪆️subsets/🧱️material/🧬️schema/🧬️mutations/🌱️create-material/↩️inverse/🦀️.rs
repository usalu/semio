//! ↩️ Inverse for `CreateMaterial` — always a `delete-material` of the created id.
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::mutations::{delete_material,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateMaterial, _base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Fem3dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: payload.material.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
