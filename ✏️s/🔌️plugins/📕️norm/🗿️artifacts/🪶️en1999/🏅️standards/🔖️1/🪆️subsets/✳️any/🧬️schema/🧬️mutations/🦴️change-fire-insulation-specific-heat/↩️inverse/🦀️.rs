//! ↩️ `change-fire-insulation-specific-heat` inverse: restores the previous `specific_heat`.

use crate::change_fire_insulation_specific_heat::ChangeFireInsulationSpecificHeat;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireInsulationSpecificHeat, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_insulation_specific_heat::ChangeFireInsulationSpecificHeat> {
    Ok(match base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) {
        Some(row) => vec![Op::from(ChangeFireInsulationSpecificHeat { fire_insulation_id: payload.fire_insulation_id.clone(), new_specific_heat: row.specific_heat })],
        None => Vec::new(),
    })
}
