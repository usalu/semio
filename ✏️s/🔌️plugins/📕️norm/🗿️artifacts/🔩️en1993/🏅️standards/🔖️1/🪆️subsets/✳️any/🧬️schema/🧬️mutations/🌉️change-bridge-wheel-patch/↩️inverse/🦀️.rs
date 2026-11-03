//! ↩️ `change-bridge-wheel-patch` inverse: restores the previous `wheel_patch`.

use crate::change_bridge_wheel_patch::ChangeBridgeWheelPatch;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeWheelPatch, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_wheel_patch::ChangeBridgeWheelPatch> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeWheelPatch { id: payload.id.clone(), new_wheel_patch: row.wheel_patch })],
        None => Vec::new(),
    })
}
