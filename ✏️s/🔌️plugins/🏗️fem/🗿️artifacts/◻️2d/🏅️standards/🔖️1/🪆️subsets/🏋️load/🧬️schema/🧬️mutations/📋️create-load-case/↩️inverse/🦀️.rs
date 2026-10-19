//! ↩️ Inverse for `CreateLoadCase` — always a `delete-load-case` of the created id.
use super::CreateLoadCase;
use crate::standards::v1::subsets::any::schema::mutations::delete_load_case;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase>>(payload: &CreateLoadCase, _base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: payload.load_case.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
