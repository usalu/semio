//! ↩️ `change-fatigue-stress-detail-id` inverse: restores the previous `detail_id`.

use crate::change_fatigue_stress_detail_id::ChangeFatigueStressDetailId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressDetailId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_detail_id::ChangeFatigueStressDetailId> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressDetailId { id: payload.id.clone(), new_detail_id: row.detail_id.clone() })],
        None => Vec::new(),
    })
}
