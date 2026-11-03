//! ↩️ `change-bridge-wheel-load` inverse: restores the previous `wheel_load`.

use crate::change_bridge_wheel_load::ChangeBridgeWheelLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeWheelLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_wheel_load::ChangeBridgeWheelLoad> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeWheelLoad { id: payload.id.clone(), new_wheel_load: row.wheel_load })],
        None => Vec::new(),
    })
}
