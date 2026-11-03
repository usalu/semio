//! ↩️ `change-silo-k` inverse: restores the previous `k`.

use crate::change_silo_k::ChangeSiloK;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloK, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_k::ChangeSiloK> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloK { id: payload.id.clone(), new_k: row.k })],
        None => Vec::new(),
    })
}
