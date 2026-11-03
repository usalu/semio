//! ↩️ `change-tower-air-density` inverse: restores the previous `air_density`.

use crate::change_tower_air_density::ChangeTowerAirDensity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerAirDensity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_air_density::ChangeTowerAirDensity> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerAirDensity { id: payload.id.clone(), new_air_density: row.air_density })],
        None => Vec::new(),
    })
}
