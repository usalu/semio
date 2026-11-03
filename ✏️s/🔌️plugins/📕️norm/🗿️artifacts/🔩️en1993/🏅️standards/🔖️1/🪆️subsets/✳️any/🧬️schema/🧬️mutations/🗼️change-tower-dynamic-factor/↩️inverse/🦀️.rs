//! ↩️ `change-tower-dynamic-factor` inverse: restores the previous `dynamic_factor`.

use crate::change_tower_dynamic_factor::ChangeTowerDynamicFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerDynamicFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_dynamic_factor::ChangeTowerDynamicFactor> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerDynamicFactor { id: payload.id.clone(), new_dynamic_factor: row.dynamic_factor })],
        None => Vec::new(),
    })
}
