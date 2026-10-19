//! ↩️ Inverse for `ReplaceSupport` — recovers the pre-mutation support from `base`.
use super::ReplaceSupport;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_support::ReplaceSupport>>(payload: &ReplaceSupport, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.supports.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceSupport { id: payload.id.clone(), new_support: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
