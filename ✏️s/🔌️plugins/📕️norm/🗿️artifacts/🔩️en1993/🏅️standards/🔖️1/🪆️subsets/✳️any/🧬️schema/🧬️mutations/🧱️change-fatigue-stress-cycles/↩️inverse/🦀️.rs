//! ↩️ `change-fatigue-stress-cycles` inverse: restores the previous `cycles`.

use crate::change_fatigue_stress_cycles::ChangeFatigueStressCycles;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressCycles, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_cycles::ChangeFatigueStressCycles> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressCycles { id: payload.id.clone(), new_cycles: row.cycles })],
        None => Vec::new(),
    })
}
