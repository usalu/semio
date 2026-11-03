//! ↩️ `change-plated-k-sigma` inverse: restores the previous `k_sigma`.

use crate::change_plated_k_sigma::ChangePlatedKSigma;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedKSigma, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_k_sigma::ChangePlatedKSigma> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedKSigma { id: payload.id.clone(), new_k_sigma: row.k_sigma })],
        None => Vec::new(),
    })
}
