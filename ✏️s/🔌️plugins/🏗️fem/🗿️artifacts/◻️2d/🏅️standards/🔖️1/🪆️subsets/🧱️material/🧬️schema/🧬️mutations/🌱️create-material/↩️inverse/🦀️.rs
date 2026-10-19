//! ↩️ Inverse for `CreateMaterial` — always a `delete-material` of the created id.
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::mutations::delete_material;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>>(payload: &CreateMaterial, _base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial { id: payload.material.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
