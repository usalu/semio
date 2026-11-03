//! ↩️ `change-wind-bridge-parapet-height` inverse: restores the previous `parapet_height`.

use crate::change_wind_bridge_parapet_height::ChangeWindBridgeParapetHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeParapetHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_parapet_height::ChangeWindBridgeParapetHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeParapetHeight, base: &En1991Snapshot) -> Option<ChangeWindBridgeParapetHeight> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeParapetHeight { id: payload.id.clone(), new_parapet_height: r0.parapet_height })
}
