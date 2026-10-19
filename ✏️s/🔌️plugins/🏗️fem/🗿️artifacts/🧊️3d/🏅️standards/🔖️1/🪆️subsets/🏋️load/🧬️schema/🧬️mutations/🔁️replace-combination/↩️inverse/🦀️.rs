//! ↩️ Inverse for `ReplaceCombination` — recovers the pre-mutation combination from `base`.
use super::ReplaceCombination;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_combination::ReplaceCombination>>(payload: &ReplaceCombination, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.combinations.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceCombination { id: payload.id.clone(), new_combination: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
