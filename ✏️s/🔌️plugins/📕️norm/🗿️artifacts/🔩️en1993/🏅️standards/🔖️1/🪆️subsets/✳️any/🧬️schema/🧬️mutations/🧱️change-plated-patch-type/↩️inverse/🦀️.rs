//! ↩️ `change-plated-patch-type` inverse: restores the previous `patch_type`.

use crate::change_plated_patch_type::ChangePlatedPatchType;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchType, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_type::ChangePlatedPatchType> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchType { id: payload.id.clone(), new_patch_type: row.patch_type })],
        None => Vec::new(),
    })
}
