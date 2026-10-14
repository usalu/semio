//! ↩️ Inverse for `CreateMaterial` — always a `delete-material` of the created id.
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::mutations::delete_material;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<delete_material::DeleteMaterial>>(payload: &CreateMaterial, _base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(delete_material::DeleteMaterial { id: payload.material.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
