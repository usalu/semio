//! ↩️ `change-fatigue-stress-fibre-z` inverse: restores the previous `fibre_z`.

use crate::change_fatigue_stress_fibre_z::ChangeFatigueStressFibreZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueStressFibreZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_stress_fibre_z::ChangeFatigueStressFibreZ> {
    Ok(match base.fatigue_stresses.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueStressFibreZ { id: payload.id.clone(), new_fibre_z: row.fibre_z })],
        None => Vec::new(),
    })
}
