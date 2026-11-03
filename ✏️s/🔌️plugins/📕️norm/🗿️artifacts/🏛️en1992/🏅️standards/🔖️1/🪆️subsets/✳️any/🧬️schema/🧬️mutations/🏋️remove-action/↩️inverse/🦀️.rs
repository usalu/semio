//! ↩️ `remove-action` inverse: inserts the removed row back at its position.

use crate::remove_action::RemoveAction;
use crate::insert_action::InsertAction;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveAction, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_action::InsertAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveAction, base: &En1992Snapshot) -> Option<InsertAction> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.actions.get(index)?;
    Some(InsertAction { member_id: payload.member_id.clone(), index, action: row.clone() })
}
