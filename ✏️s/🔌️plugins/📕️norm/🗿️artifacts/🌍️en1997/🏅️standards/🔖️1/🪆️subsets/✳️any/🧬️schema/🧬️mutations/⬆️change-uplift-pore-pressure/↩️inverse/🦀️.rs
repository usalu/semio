//! ↩️ `change-uplift-pore-pressure` inverse: restores the previous `pore_pressure`.

use crate::change_uplift_pore_pressure::ChangeUpliftPorePressure;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeUpliftPorePressure, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uplift_pore_pressure::ChangeUpliftPorePressure> {
    Ok(match base.uplift_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeUpliftPorePressure { id: payload.id.clone(), new_pore_pressure: row.pore_pressure })],
        None => Vec::new(),
    })
}
