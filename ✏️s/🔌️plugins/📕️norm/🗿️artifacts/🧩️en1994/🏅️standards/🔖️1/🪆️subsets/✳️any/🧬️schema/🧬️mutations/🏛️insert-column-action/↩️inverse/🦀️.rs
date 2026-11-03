//! ↩️ `insert-column-action` inverse: removes the inserted row again.

use crate::insert_column_action::InsertColumnAction;
use crate::remove_column_action::RemoveColumnAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &InsertColumnAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_column_action::RemoveColumnAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertColumnAction, base: &En1994Snapshot) -> Option<RemoveColumnAction> {
    let _r0 = base.columns.iter().find(|row| row.id == payload.column_id)?;
    Some(RemoveColumnAction { column_id: payload.column_id.clone(), id: payload.action.id.clone() })
}
