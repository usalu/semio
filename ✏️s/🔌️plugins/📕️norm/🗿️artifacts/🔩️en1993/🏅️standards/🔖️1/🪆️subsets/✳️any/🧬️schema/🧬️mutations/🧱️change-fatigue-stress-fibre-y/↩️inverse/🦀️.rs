//! ↩️ `change-fatigue-stress-fibre-y` inverse: restores the previous `fibre_y`.

use crate::change_fatigue_stress_fibre_y::ChangeFatigueStressFibreY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressFibreY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_fibre_y::ChangeFatigueStressFibreY> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressFibreY { id: payload.id.clone(), new_fibre_y: row.fibre_y })],
        None => Vec::new(),
    })
}
