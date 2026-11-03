//! ↩️ `change-plated-patch-moment` inverse: restores the previous `patch_moment`.

use crate::change_plated_patch_moment::ChangePlatedPatchMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_moment::ChangePlatedPatchMoment> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchMoment { id: payload.id.clone(), new_patch_moment: row.patch_moment })],
        None => Vec::new(),
    })
}
