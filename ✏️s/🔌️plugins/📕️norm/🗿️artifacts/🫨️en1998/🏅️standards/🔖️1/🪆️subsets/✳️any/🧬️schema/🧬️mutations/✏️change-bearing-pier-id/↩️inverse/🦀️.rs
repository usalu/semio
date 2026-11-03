//! ↩️ `change-bearing-pier-id` inverse: restores the previous `pier_id`.

use crate::change_bearing_pier_id::ChangeBearingPierId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingPierId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_pier_id::ChangeBearingPierId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingPierId, base: &En1998Snapshot) -> Option<ChangeBearingPierId> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingPierId { id: payload.id.clone(), new_pier_id: r0.pier_id.clone() })
}
