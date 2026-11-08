//! ↩️ `insert-sheet-profile` inverse.

use crate::insert_sheet_profile::InsertSheetProfile;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_sheet_profile;

pub fn inverse<Op>(payload: &InsertSheetProfile, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_sheet_profile::RemoveSheetProfile> {
    Ok((|| {
    vec![Op::from(remove_sheet_profile::RemoveSheetProfile { id: payload.sheet_profile.id.clone() })]

    })())
}
