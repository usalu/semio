//! ↩️ `change-water-well-capacity` inverse: restores the previous `well_capacity`.

use crate::change_water_well_capacity::ChangeWaterWellCapacity;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterWellCapacity, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_well_capacity::ChangeWaterWellCapacity> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterWellCapacity { id: payload.id.clone(), new_well_capacity: row.well_capacity })],
        None => Vec::new(),
    })
}
