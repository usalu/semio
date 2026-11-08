//! ↩️ `change-sheet-profile-pitch` inverse: restores the previous `pitch`.

use crate::change_sheet_profile_pitch::ChangeSheetProfilePitch;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetProfilePitch, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_profile_pitch::ChangeSheetProfilePitch> {
    Ok(match base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) {
        Some(row) => vec![Op::from(ChangeSheetProfilePitch { sheet_profile_id: payload.sheet_profile_id.clone(), new_pitch: row.pitch })],
        None => Vec::new(),
    })
}
