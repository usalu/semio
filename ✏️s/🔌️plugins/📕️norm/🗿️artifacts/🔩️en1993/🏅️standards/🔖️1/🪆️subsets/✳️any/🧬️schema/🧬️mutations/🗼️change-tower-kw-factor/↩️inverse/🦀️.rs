//! ↩️ `change-tower-kw-factor` inverse: restores the previous `kw_factor`.

use crate::change_tower_kw_factor::ChangeTowerKwFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerKwFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_kw_factor::ChangeTowerKwFactor> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerKwFactor { id: payload.id.clone(), new_kw_factor: row.kw_factor })],
        None => Vec::new(),
    })
}
