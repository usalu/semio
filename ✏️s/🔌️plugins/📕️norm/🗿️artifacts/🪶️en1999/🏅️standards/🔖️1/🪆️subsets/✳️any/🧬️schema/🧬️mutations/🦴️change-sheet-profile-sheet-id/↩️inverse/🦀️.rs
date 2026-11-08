//! ↩️ `change-sheet-profile-sheet-id` inverse: restores the previous `sheet_id`.

use crate::change_sheet_profile_sheet_id::ChangeSheetProfileSheetId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetProfileSheetId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_profile_sheet_id::ChangeSheetProfileSheetId> {
    Ok(match base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) {
        Some(row) => vec![Op::from(ChangeSheetProfileSheetId { sheet_profile_id: payload.sheet_profile_id.clone(), new_sheet_id: row.sheet_id.clone() })],
        None => Vec::new(),
    })
}
