//! ↩️ `change-pier-base-pier-id` inverse: restores the previous `pier_id`.

use crate::change_pier_base_pier_id::ChangePierBasePierId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierBasePierId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_base_pier_id::ChangePierBasePierId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierBasePierId, base: &En1998Snapshot) -> Option<ChangePierBasePierId> {
    let r0 = base.pier_bases.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierBasePierId { id: payload.id.clone(), new_pier_id: r0.pier_id.clone() })
}
