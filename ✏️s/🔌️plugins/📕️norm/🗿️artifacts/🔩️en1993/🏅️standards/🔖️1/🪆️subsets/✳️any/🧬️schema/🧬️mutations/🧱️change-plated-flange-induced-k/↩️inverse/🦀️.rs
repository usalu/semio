//! ↩️ `change-plated-flange-induced-k` inverse: restores the previous `flange_induced_k`.

use crate::change_plated_flange_induced_k::ChangePlatedFlangeInducedK;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeInducedK, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_induced_k::ChangePlatedFlangeInducedK> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeInducedK { id: payload.id.clone(), new_flange_induced_k: row.flange_induced_k })],
        None => Vec::new(),
    })
}
