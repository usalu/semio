//! ↩️ `change-fatigue-stress-history` inverse: restores the previous `stress_history`.

use crate::change_fatigue_stress_history::ChangeFatigueStressHistory;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressHistory, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_history::ChangeFatigueStressHistory> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressHistory { id: payload.id.clone(), new_stress_history: row.stress_history.clone() })],
        None => Vec::new(),
    })
}
