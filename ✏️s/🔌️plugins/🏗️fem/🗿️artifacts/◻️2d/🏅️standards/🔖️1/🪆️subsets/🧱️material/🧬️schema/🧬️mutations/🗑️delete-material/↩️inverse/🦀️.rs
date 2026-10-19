//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::create_material;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial>>(payload: &DeleteMaterial, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial { material: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
