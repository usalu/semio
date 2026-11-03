//! ↩️ `change-wall-earth-pressure-mode` inverse: restores the previous `earth_pressure_mode`.

use crate::change_wall_earth_pressure_mode::ChangeWallEarthPressureMode;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallEarthPressureMode, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_earth_pressure_mode::ChangeWallEarthPressureMode> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallEarthPressureMode { id: payload.id.clone(), new_earth_pressure_mode: row.earth_pressure_mode })],
        None => Vec::new(),
    })
}
