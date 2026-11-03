//! ↩️ `change-water-well-water-depth` inverse: restores the previous `well_water_depth`.

use crate::change_water_well_water_depth::ChangeWaterWellWaterDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterWellWaterDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_well_water_depth::ChangeWaterWellWaterDepth> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterWellWaterDepth { id: payload.id.clone(), new_well_water_depth: row.well_water_depth })],
        None => Vec::new(),
    })
}
