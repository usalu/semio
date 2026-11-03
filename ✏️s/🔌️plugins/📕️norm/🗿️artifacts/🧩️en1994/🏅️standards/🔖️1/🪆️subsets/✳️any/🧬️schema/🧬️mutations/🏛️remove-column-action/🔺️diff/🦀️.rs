//! 🏛️ `remove-column-action` diff on the keyed `actions` lane of `columns`.

use crate::remove_column_action::RemoveColumnAction;
use semio_s_artifact_norm_en1994_diff::{En1994ColumnsActionsRemoved, En1994ColumnsActionsRows, En1994ColumnsPatch, En1994ColumnsRows, En1994Diff};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &RemoveColumnAction, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(parent) = base.columns.iter().find(|row| row.id == payload.column_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column {} does not exist.", payload.column_id), [payload.column_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows::modification(&payload.column_id, En1994ColumnsPatch { actions: Some(En1994ColumnsActionsRows::removal(&parent.actions, index)), ..Default::default() })), ..Default::default() })
}
