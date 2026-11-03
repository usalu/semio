//! ↩️ `change-tower-k-factor` inverse: restores the previous `k_factor`.

use crate::change_tower_k_factor::ChangeTowerKFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerKFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_k_factor::ChangeTowerKFactor> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerKFactor { id: payload.id.clone(), new_k_factor: row.k_factor })],
        None => Vec::new(),
    })
}
