//! ↩️ `change-crane-damage-exponent` inverse: restores the previous `damage_exponent`.

use crate::change_crane_damage_exponent::ChangeCraneDamageExponent;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneDamageExponent, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_damage_exponent::ChangeCraneDamageExponent> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneDamageExponent { id: payload.id.clone(), new_damage_exponent: row.damage_exponent })],
        None => Vec::new(),
    })
}
