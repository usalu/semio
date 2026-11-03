//! ↩️ `change-water-piezometric-depth` inverse: restores the previous `piezometric_depth`.

use crate::change_water_piezometric_depth::ChangeWaterPiezometricDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterPiezometricDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_piezometric_depth::ChangeWaterPiezometricDepth> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterPiezometricDepth { id: payload.id.clone(), new_piezometric_depth: row.piezometric_depth })],
        None => Vec::new(),
    })
}
