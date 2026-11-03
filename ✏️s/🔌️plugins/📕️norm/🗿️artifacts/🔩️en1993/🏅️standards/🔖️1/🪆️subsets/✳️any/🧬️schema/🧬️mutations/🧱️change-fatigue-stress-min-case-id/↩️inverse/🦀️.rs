//! ↩️ `change-fatigue-stress-min-case-id` inverse: restores the previous `min_case_id`.

use crate::change_fatigue_stress_min_case_id::ChangeFatigueStressMinCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressMinCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_min_case_id::ChangeFatigueStressMinCaseId> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressMinCaseId { id: payload.id.clone(), new_min_case_id: row.min_case_id.clone() })],
        None => Vec::new(),
    })
}
