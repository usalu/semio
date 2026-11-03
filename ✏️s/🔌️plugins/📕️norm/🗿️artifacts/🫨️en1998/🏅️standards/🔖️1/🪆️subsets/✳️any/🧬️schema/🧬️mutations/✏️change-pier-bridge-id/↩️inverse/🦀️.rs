//! ↩️ `change-pier-bridge-id` inverse: restores the previous `bridge_id`.

use crate::change_pier_bridge_id::ChangePierBridgeId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBridgeId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_bridge_id::ChangePierBridgeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBridgeId, base: &En1998Snapshot) -> Option<ChangePierBridgeId> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBridgeId { id: payload.id.clone(), new_bridge_id: r0.bridge_id.clone() })
}
