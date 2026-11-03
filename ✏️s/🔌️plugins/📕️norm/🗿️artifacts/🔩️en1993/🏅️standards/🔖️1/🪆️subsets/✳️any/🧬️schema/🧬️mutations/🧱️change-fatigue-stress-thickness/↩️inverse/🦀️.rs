//! ↩️ `change-fatigue-stress-thickness` inverse: restores the previous `thickness`.

use crate::change_fatigue_stress_thickness::ChangeFatigueStressThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_thickness::ChangeFatigueStressThickness> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressThickness { id: payload.id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
