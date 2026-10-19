//! ↩️ Inverse for `DeleteLoadCase` — recreates the captured load case from `base`.
use super::DeleteLoadCase;
use crate::standards::v1::subsets::any::schema::mutations::create_load_case;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase>>(payload: &DeleteLoadCase, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase { load_case: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
