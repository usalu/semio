//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::{create_material,Fem2dMutation};

use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem2dMutation::CreateMaterial(create_material::CreateMaterial { material: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
