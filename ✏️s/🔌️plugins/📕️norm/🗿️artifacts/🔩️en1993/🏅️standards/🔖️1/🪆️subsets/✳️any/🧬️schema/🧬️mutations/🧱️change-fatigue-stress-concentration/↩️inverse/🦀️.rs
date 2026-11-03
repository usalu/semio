//! ↩️ `change-fatigue-stress-concentration` inverse: restores the previous `concentration`.

use crate::change_fatigue_stress_concentration::ChangeFatigueStressConcentration;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressConcentration, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_concentration::ChangeFatigueStressConcentration> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressConcentration { id: payload.id.clone(), new_concentration: row.concentration })],
        None => Vec::new(),
    })
}
