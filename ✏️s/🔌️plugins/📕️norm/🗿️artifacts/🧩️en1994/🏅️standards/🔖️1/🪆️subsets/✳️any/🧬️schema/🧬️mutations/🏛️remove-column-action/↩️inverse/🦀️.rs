//! ↩️ `remove-column-action` inverse: inserts the removed row back at its position.

use crate::remove_column_action::RemoveColumnAction;
use crate::insert_column_action::InsertColumnAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &RemoveColumnAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_column_action::InsertColumnAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveColumnAction, base: &En1994Snapshot) -> Option<InsertColumnAction> {
    let r0 = base.columns.iter().find(|row| row.id == payload.column_id)?;
    let (index, row) = r0.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertColumnAction { column_id: payload.column_id.clone(), index, action: row.clone() })
}
