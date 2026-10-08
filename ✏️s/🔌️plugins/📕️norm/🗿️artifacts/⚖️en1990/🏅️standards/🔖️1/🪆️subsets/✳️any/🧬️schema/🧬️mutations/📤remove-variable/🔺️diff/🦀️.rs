//! 📤 `remove-variable` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveVariable;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990VariableEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveVariable, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.variables.len() {
        return MutationOutcome::error("mutation.target-missing", "variables index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { variables: vec![En1990VariableEdit::remove(payload.index, base.variables[payload.index].id.clone())], ..En1990Diff::default() })
}
