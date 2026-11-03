//! ↩️ `change-tower-mass-per-length` inverse: restores the previous `mass_per_length`.

use crate::change_tower_mass_per_length::ChangeTowerMassPerLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerMassPerLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_mass_per_length::ChangeTowerMassPerLength> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerMassPerLength { id: payload.id.clone(), new_mass_per_length: row.mass_per_length })],
        None => Vec::new(),
    })
}
