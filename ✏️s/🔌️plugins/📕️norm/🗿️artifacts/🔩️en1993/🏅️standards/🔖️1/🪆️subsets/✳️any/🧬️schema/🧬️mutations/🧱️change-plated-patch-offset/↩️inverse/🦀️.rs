//! ↩️ `change-plated-patch-offset` inverse: restores the previous `patch_offset`.

use crate::change_plated_patch_offset::ChangePlatedPatchOffset;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchOffset, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_offset::ChangePlatedPatchOffset> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchOffset { id: payload.id.clone(), new_patch_offset: row.patch_offset })],
        None => Vec::new(),
    })
}
