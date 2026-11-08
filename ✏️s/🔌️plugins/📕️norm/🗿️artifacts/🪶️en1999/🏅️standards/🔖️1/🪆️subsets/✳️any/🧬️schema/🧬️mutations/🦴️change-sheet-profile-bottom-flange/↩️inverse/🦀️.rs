//! ↩️ `change-sheet-profile-bottom-flange` inverse: restores the previous `bottom_flange`.

use crate::change_sheet_profile_bottom_flange::ChangeSheetProfileBottomFlange;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetProfileBottomFlange, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_profile_bottom_flange::ChangeSheetProfileBottomFlange> {
    Ok(match base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) {
        Some(row) => vec![Op::from(ChangeSheetProfileBottomFlange { sheet_profile_id: payload.sheet_profile_id.clone(), new_bottom_flange: row.bottom_flange })],
        None => Vec::new(),
    })
}
