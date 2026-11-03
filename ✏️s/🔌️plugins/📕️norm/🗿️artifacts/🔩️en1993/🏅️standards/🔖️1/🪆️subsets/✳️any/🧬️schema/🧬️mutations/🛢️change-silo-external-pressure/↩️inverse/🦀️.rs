//! ↩️ `change-silo-external-pressure` inverse: restores the previous `external_pressure`.

use crate::change_silo_external_pressure::ChangeSiloExternalPressure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloExternalPressure, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_external_pressure::ChangeSiloExternalPressure> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloExternalPressure { id: payload.id.clone(), new_external_pressure: row.external_pressure })],
        None => Vec::new(),
    })
}
