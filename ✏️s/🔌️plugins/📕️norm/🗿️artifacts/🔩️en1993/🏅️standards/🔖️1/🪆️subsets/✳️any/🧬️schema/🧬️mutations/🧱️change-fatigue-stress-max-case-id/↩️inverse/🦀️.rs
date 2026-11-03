//! ↩️ `change-fatigue-stress-max-case-id` inverse: restores the previous `max_case_id`.

use crate::change_fatigue_stress_max_case_id::ChangeFatigueStressMaxCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressMaxCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_max_case_id::ChangeFatigueStressMaxCaseId> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressMaxCaseId { id: payload.id.clone(), new_max_case_id: row.max_case_id.clone() })],
        None => Vec::new(),
    })
}
