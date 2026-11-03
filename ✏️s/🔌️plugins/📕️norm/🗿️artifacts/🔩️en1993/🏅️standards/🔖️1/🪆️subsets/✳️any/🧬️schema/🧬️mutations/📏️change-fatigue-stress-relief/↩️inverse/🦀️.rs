//! ↩️ `change-fatigue-stress-relief` inverse: restores the previous `relief`.

use crate::change_fatigue_stress_relief::ChangeFatigueStressRelief;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressRelief, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_relief::ChangeFatigueStressRelief> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressRelief { id: payload.id.clone(), new_relief: row.relief })],
        None => Vec::new(),
    })
}
