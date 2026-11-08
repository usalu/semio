//! ↩️ `change-fire-insulation-conductivity` inverse: restores the previous `conductivity`.

use crate::change_fire_insulation_conductivity::ChangeFireInsulationConductivity;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireInsulationConductivity, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_insulation_conductivity::ChangeFireInsulationConductivity> {
    Ok(match base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) {
        Some(row) => vec![Op::from(ChangeFireInsulationConductivity { fire_insulation_id: payload.fire_insulation_id.clone(), new_conductivity: row.conductivity })],
        None => Vec::new(),
    })
}
