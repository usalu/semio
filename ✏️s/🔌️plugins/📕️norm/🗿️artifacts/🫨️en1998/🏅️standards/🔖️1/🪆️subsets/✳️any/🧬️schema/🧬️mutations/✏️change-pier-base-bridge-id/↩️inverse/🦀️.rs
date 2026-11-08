//! ↩️ `change-pier-base-bridge-id` inverse: restores the previous `bridge_id`.

use crate::change_pier_base_bridge_id::ChangePierBaseBridgeId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBaseBridgeId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_bridge_id::ChangePierBaseBridgeId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBaseBridgeId, base: &En1998Snapshot) -> Option<ChangePierBaseBridgeId> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBaseBridgeId { id: payload.id.clone(), new_bridge_id: r0.bridge_id.clone() })
}
