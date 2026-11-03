//! ↩️ `insert-action` inverse: removes the inserted row again.

use crate::insert_action::InsertAction;
use crate::remove_action::RemoveAction;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertAction, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_action::RemoveAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertAction, base: &En1992Snapshot) -> Option<RemoveAction> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveAction { member_id: payload.member_id.clone(), index })
}
