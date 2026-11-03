//! ↩️ `change-wind-bridge-ground-height` inverse: restores the previous `height_above_ground`.

use crate::change_wind_bridge_ground_height::ChangeWindBridgeGroundHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeGroundHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_ground_height::ChangeWindBridgeGroundHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeGroundHeight, base: &En1991Snapshot) -> Option<ChangeWindBridgeGroundHeight> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeGroundHeight { id: payload.id.clone(), new_height_above_ground: r0.height_above_ground })
}
