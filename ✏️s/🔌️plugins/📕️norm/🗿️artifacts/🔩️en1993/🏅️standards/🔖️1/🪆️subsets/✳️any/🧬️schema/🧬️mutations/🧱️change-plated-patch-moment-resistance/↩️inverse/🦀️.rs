//! ↩️ `change-plated-patch-moment-resistance` inverse: restores the previous `patch_moment_resistance`.

use crate::change_plated_patch_moment_resistance::ChangePlatedPatchMomentResistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPatchMomentResistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_patch_moment_resistance::ChangePlatedPatchMomentResistance> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPatchMomentResistance { id: payload.id.clone(), new_patch_moment_resistance: row.patch_moment_resistance })],
        None => Vec::new(),
    })
}
