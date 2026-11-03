//! ↩️ `change-bearing-bridge-id` inverse: restores the previous `bridge_id`.

use crate::change_bearing_bridge_id::ChangeBearingBridgeId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingBridgeId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_bridge_id::ChangeBearingBridgeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingBridgeId, base: &En1998Snapshot) -> Option<ChangeBearingBridgeId> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingBridgeId { id: payload.id.clone(), new_bridge_id: r0.bridge_id.clone() })
}
