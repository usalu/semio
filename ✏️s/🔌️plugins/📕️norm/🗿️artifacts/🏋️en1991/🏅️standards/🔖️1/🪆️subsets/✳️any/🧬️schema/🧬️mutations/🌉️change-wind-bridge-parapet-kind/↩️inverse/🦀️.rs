//! ↩️ `change-wind-bridge-parapet-kind` inverse: restores the previous `parapet_kind`.

use crate::change_wind_bridge_parapet_kind::ChangeWindBridgeParapetKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeParapetKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_parapet_kind::ChangeWindBridgeParapetKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeParapetKind, base: &En1991Snapshot) -> Option<ChangeWindBridgeParapetKind> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeParapetKind { id: payload.id.clone(), new_parapet_kind: r0.parapet_kind })
}
