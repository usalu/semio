//! ↩️ `insert-beam-action` inverse: removes the inserted row again.

use crate::insert_beam_action::InsertBeamAction;
use crate::remove_beam_action::RemoveBeamAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &InsertBeamAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_beam_action::RemoveBeamAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBeamAction, base: &En1994Snapshot) -> Option<RemoveBeamAction> {
    let _r0 = base.beams.iter().find(|row| row.id == payload.beam_id)?;
    Some(RemoveBeamAction { beam_id: payload.beam_id.clone(), id: payload.action.id.clone() })
}
