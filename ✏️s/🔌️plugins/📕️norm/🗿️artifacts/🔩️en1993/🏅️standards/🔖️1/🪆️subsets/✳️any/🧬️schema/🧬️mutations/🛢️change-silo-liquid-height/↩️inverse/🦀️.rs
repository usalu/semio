//! ↩️ `change-silo-liquid-height` inverse: restores the previous `liquid_height`.

use crate::change_silo_liquid_height::ChangeSiloLiquidHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloLiquidHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_liquid_height::ChangeSiloLiquidHeight> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloLiquidHeight { id: payload.id.clone(), new_liquid_height: row.liquid_height })],
        None => Vec::new(),
    })
}
