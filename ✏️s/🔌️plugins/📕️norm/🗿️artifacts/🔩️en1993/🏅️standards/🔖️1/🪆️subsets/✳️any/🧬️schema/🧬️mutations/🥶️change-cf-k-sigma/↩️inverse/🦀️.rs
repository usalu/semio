//! ↩️ `change-cf-k-sigma` inverse: restores the previous `k_sigma`.

use crate::change_cf_k_sigma::ChangeCfKSigma;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfKSigma, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_k_sigma::ChangeCfKSigma> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfKSigma { id: payload.id.clone(), new_k_sigma: row.k_sigma })],
        None => Vec::new(),
    })
}
