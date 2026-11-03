//! ↩️ `change-wind-bridge-traffic-kind` inverse: restores the previous `traffic_kind`.

use crate::change_wind_bridge_traffic_kind::ChangeWindBridgeTrafficKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeTrafficKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_traffic_kind::ChangeWindBridgeTrafficKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeTrafficKind, base: &En1991Snapshot) -> Option<ChangeWindBridgeTrafficKind> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeTrafficKind { id: payload.id.clone(), new_traffic_kind: r0.traffic_kind })
}
