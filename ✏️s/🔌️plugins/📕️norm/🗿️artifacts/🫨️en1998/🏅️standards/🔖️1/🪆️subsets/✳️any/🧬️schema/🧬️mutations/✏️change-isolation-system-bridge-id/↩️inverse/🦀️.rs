//! ↩️ `change-isolation-system-bridge-id` inverse: restores the previous `bridge_id`.

use crate::change_isolation_system_bridge_id::ChangeIsolationSystemBridgeId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemBridgeId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_bridge_id::ChangeIsolationSystemBridgeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemBridgeId, base: &En1998Snapshot) -> Option<ChangeIsolationSystemBridgeId> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemBridgeId { id: payload.id.clone(), new_bridge_id: r0.bridge_id.clone() })
}
