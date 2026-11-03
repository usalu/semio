//! ↩️ `change-silo-internal-pressure` inverse: restores the previous `internal_pressure`.

use crate::change_silo_internal_pressure::ChangeSiloInternalPressure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloInternalPressure, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_internal_pressure::ChangeSiloInternalPressure> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloInternalPressure { id: payload.id.clone(), new_internal_pressure: row.internal_pressure })],
        None => Vec::new(),
    })
}
