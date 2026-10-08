//! ↩️ Inverse for `DeleteSolid` — recreates the captured solid from `base`.
use super::DeleteSolid;
use crate::standards::v1::subsets::any::schema::mutations::{create_solid,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSolid, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.solids.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem3dMutation::CreateSolid(create_solid::CreateSolid { solid: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
