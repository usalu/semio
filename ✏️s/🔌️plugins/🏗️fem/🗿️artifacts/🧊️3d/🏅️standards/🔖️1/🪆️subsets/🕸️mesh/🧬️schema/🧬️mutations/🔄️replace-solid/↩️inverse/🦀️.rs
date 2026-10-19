//! ↩️ Inverse for `ReplaceSolid` — recovers the pre-mutation solid from `base`.
use super::ReplaceSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_solid::ReplaceSolid>>(payload: &ReplaceSolid, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.solids.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceSolid { id: payload.id.clone(), new_solid: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
