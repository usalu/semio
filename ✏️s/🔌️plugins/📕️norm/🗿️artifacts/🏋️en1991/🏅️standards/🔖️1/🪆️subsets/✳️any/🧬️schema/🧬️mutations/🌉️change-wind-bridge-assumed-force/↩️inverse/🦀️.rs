//! ↩️ `change-wind-bridge-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_wind_bridge_assumed_force::ChangeWindBridgeAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_assumed_force::ChangeWindBridgeAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeAssumedForce, base: &En1991Snapshot) -> Option<ChangeWindBridgeAssumedForce> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
