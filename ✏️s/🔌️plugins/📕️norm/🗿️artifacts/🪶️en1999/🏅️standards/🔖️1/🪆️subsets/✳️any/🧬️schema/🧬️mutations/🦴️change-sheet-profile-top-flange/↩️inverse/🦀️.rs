//! ↩️ `change-sheet-profile-top-flange` inverse: restores the previous `top_flange`.

use crate::change_sheet_profile_top_flange::ChangeSheetProfileTopFlange;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetProfileTopFlange, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_profile_top_flange::ChangeSheetProfileTopFlange> {
    Ok(match base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) {
        Some(row) => vec![Op::from(ChangeSheetProfileTopFlange { sheet_profile_id: payload.sheet_profile_id.clone(), new_top_flange: row.top_flange })],
        None => Vec::new(),
    })
}
