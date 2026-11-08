//! ↩️ `remove-sheet-profile` inverse.

use crate::remove_sheet_profile::RemoveSheetProfile;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_sheet_profile;

pub fn inverse<Op>(payload: &RemoveSheetProfile, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_sheet_profile::InsertSheetProfile> {
    Ok((|| {
    if let Some((index, row)) = base.sheet_profiles.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_sheet_profile::InsertSheetProfile { index: index as u32, sheet_profile: row.clone() })]
    } else { Vec::new() }

    })())
}
