//! ↩️ `change-cf-psi` inverse: restores the previous `psi`.

use crate::change_cf_psi::ChangeCfPsi;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfPsi, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_psi::ChangeCfPsi> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfPsi { id: payload.id.clone(), new_psi: row.psi })],
        None => Vec::new(),
    })
}
