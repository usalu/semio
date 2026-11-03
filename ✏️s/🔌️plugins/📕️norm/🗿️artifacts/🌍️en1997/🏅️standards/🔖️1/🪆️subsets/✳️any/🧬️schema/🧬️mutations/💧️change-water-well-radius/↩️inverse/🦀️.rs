//! ↩️ `change-water-well-radius` inverse: restores the previous `well_radius`.

use crate::change_water_well_radius::ChangeWaterWellRadius;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWaterWellRadius, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_water_well_radius::ChangeWaterWellRadius> {
    Ok(match base.layer_waters.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWaterWellRadius { id: payload.id.clone(), new_well_radius: row.well_radius })],
        None => Vec::new(),
    })
}
