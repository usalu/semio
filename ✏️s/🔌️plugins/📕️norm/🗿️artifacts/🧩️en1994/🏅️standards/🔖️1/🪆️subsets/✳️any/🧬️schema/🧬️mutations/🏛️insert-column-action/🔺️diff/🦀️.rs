//! 🏛️ `insert-column-action` diff on the keyed `actions` lane of `columns`.

use crate::insert_column_action::InsertColumnAction;
use semio_s_artifact_norm_en1994_diff::{En1994ColumnsActionsInserted, En1994ColumnsActionsRows, En1994ColumnsPatch, En1994ColumnsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &InsertColumnAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.columns.iter().find(|row| row.id == payload.column_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column {} does not exist.", payload.column_id), [payload.column_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        let key = payload.action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Column action {key} already exists."), [key]);
    }
    if payload.index > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows::modification(&payload.column_id, En1994ColumnsPatch { actions: Some(En1994ColumnsActionsRows::insertion(payload.index, payload.action.clone())), ..Default::default() })), ..Default::default() })
}
