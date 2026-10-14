//! ↩️ Inverse for `DeleteSolid` — recreates the captured solid from `base`.
use super::DeleteSolid;
use crate::standards::v1::subsets::any::schema::mutations::create_solid;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_solid::CreateSolid>>(payload: &DeleteSolid, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.solids.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(create_solid::CreateSolid { solid: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
