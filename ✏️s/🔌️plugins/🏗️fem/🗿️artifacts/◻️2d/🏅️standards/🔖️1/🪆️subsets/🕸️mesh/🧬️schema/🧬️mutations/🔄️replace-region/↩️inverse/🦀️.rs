//! ↩️ Inverse for `ReplaceRegion` — recovers the pre-mutation region from `base`.
use super::ReplaceRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion>>(payload: &ReplaceRegion, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.regions.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceRegion { id: payload.id.clone(), new_region: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
