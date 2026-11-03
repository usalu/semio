//! ↩️ `change-tower-force-coefficient` inverse: restores the previous `force_coefficient`.

use crate::change_tower_force_coefficient::ChangeTowerForceCoefficient;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerForceCoefficient, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_force_coefficient::ChangeTowerForceCoefficient> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerForceCoefficient { id: payload.id.clone(), new_force_coefficient: row.force_coefficient })],
        None => Vec::new(),
    })
}
