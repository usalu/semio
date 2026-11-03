//! ↩️ `change-plated-lateral-pressure` inverse: restores the previous `lateral_pressure`.

use crate::change_plated_lateral_pressure::ChangePlatedLateralPressure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedLateralPressure, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_lateral_pressure::ChangePlatedLateralPressure> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedLateralPressure { id: payload.id.clone(), new_lateral_pressure: row.lateral_pressure })],
        None => Vec::new(),
    })
}
