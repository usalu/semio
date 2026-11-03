//! ↩️ `change-crane-wheel-eccentricity` inverse: restores the previous `wheel_eccentricity`.

use crate::change_crane_wheel_eccentricity::ChangeCraneWheelEccentricity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneWheelEccentricity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_wheel_eccentricity::ChangeCraneWheelEccentricity> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneWheelEccentricity { id: payload.id.clone(), new_wheel_eccentricity: row.wheel_eccentricity })],
        None => Vec::new(),
    })
}
