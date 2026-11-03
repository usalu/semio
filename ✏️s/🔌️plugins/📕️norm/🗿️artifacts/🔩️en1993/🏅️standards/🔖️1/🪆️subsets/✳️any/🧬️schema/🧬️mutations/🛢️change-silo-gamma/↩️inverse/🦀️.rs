//! ↩️ `change-silo-gamma` inverse: restores the previous `gamma`.

use crate::change_silo_gamma::ChangeSiloGamma;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloGamma, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_gamma::ChangeSiloGamma> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloGamma { id: payload.id.clone(), new_gamma: row.gamma })],
        None => Vec::new(),
    })
}
