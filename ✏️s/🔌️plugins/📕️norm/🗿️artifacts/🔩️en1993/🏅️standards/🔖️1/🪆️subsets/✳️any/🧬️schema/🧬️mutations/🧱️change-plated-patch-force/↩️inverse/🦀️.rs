//! ↩️ `change-plated-patch-force` inverse: restores the previous `patch_force`.

use crate::change_plated_patch_force::ChangePlatedPatchForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_force::ChangePlatedPatchForce> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchForce { id: payload.id.clone(), new_patch_force: row.patch_force })],
        None => Vec::new(),
    })
}
