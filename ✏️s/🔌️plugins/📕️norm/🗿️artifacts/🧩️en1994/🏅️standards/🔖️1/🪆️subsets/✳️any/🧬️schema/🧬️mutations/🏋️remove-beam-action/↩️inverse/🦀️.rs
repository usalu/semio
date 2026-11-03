//! ↩️ `remove-beam-action` inverse: inserts the removed row back at its position.

use crate::remove_beam_action::RemoveBeamAction;
use crate::insert_beam_action::InsertBeamAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &RemoveBeamAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_beam_action::InsertBeamAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBeamAction, base: &En1994Snapshot) -> Option<InsertBeamAction> {
    let r0 = base.beams.iter().find(|row| row.id == payload.beam_id)?;
    let (index, row) = r0.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBeamAction { beam_id: payload.beam_id.clone(), index, action: row.clone() })
}
