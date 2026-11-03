//! ↩️ `change-fire-mu0` inverse: restores the previous `mu0`.

use crate::change_fire_mu0::ChangeFireMu0;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireMu0, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_mu0::ChangeFireMu0> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireMu0 { id: payload.id.clone(), new_mu0: row.mu0 })],
        None => Vec::new(),
    })
}
