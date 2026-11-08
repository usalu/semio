//! ↩️ `change-sheet-profile-web-height` inverse: restores the previous `web_height`.

use crate::change_sheet_profile_web_height::ChangeSheetProfileWebHeight;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetProfileWebHeight, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_profile_web_height::ChangeSheetProfileWebHeight> {
    Ok(match base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) {
        Some(row) => vec![Op::from(ChangeSheetProfileWebHeight { sheet_profile_id: payload.sheet_profile_id.clone(), new_web_height: row.web_height })],
        None => Vec::new(),
    })
}
