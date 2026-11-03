//! ↩️ `change-plated-stress-x` inverse: restores the previous `stress_x`.

use crate::change_plated_stress_x::ChangePlatedStressX;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStressX, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stress_x::ChangePlatedStressX> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStressX { id: payload.id.clone(), new_stress_x: row.stress_x })],
        None => Vec::new(),
    })
}
