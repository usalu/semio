//! ↩️ `change-diaphragm-storey-id` inverse: restores the previous `storey_id`.

use crate::change_diaphragm_storey_id::ChangeDiaphragmStoreyId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmStoreyId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_storey_id::ChangeDiaphragmStoreyId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmStoreyId, base: &En1998Snapshot) -> Option<ChangeDiaphragmStoreyId> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmStoreyId { id: payload.id.clone(), new_storey_id: r0.storey_id.clone() })
}
