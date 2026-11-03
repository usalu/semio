//! ↩️ `change-crane-max-wheel-load` inverse: restores the previous `max_wheel_load`.

use crate::change_crane_max_wheel_load::ChangeCraneMaxWheelLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneMaxWheelLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_max_wheel_load::ChangeCraneMaxWheelLoad> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneMaxWheelLoad { id: payload.id.clone(), new_max_wheel_load: row.max_wheel_load })],
        None => Vec::new(),
    })
}
