//! ↩️ `change-plated-patch-length` inverse: restores the previous `patch_length`.

use crate::change_plated_patch_length::ChangePlatedPatchLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_length::ChangePlatedPatchLength> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchLength { id: payload.id.clone(), new_patch_length: row.patch_length })],
        None => Vec::new(),
    })
}
