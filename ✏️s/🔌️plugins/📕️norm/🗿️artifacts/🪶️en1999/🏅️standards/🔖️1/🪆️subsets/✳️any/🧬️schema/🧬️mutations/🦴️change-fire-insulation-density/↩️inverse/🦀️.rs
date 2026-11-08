//! ↩️ `change-fire-insulation-density` inverse: restores the previous `density`.

use crate::change_fire_insulation_density::ChangeFireInsulationDensity;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireInsulationDensity, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_insulation_density::ChangeFireInsulationDensity> {
    Ok(match base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) {
        Some(row) => vec![Op::from(ChangeFireInsulationDensity { fire_insulation_id: payload.fire_insulation_id.clone(), new_density: row.density })],
        None => Vec::new(),
    })
}
