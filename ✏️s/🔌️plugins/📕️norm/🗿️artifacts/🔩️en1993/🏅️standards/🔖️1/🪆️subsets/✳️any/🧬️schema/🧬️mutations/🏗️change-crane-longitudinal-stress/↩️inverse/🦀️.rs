//! ↩️ `change-crane-longitudinal-stress` inverse: restores the previous `longitudinal_stress`.

use crate::change_crane_longitudinal_stress::ChangeCraneLongitudinalStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneLongitudinalStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_longitudinal_stress::ChangeCraneLongitudinalStress> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneLongitudinalStress { id: payload.id.clone(), new_longitudinal_stress: row.longitudinal_stress })],
        None => Vec::new(),
    })
}
