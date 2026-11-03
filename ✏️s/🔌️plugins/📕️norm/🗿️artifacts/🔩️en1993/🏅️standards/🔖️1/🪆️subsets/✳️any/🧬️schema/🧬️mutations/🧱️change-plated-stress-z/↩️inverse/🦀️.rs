//! ↩️ `change-plated-stress-z` inverse: restores the previous `stress_z`.

use crate::change_plated_stress_z::ChangePlatedStressZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStressZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stress_z::ChangePlatedStressZ> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStressZ { id: payload.id.clone(), new_stress_z: row.stress_z })],
        None => Vec::new(),
    })
}
