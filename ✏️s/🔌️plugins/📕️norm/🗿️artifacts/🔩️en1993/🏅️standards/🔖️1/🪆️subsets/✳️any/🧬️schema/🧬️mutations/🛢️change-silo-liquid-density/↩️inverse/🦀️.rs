//! ↩️ `change-silo-liquid-density` inverse: restores the previous `liquid_density`.

use crate::change_silo_liquid_density::ChangeSiloLiquidDensity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloLiquidDensity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_liquid_density::ChangeSiloLiquidDensity> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloLiquidDensity { id: payload.id.clone(), new_liquid_density: row.liquid_density })],
        None => Vec::new(),
    })
}
