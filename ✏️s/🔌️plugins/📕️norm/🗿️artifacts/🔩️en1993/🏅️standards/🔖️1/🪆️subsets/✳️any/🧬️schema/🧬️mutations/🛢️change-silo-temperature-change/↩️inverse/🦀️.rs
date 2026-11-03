//! ↩️ `change-silo-temperature-change` inverse: restores the previous `temperature_change`.

use crate::change_silo_temperature_change::ChangeSiloTemperatureChange;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloTemperatureChange, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_temperature_change::ChangeSiloTemperatureChange> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloTemperatureChange { id: payload.id.clone(), new_temperature_change: row.temperature_change })],
        None => Vec::new(),
    })
}
