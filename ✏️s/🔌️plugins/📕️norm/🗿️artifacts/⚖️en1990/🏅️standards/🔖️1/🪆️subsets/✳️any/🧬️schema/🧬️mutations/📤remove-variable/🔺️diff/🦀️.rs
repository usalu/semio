//! 📤 `remove-variable` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveVariable;
use crate::diff::{En1990Diff, En1990VariableDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveVariable, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.variables.len() {
        return MutationOutcome::error("mutation.target-missing", "variables index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { variables: En1990VariableDelta::removal(&base.variables, payload.index), ..En1990Diff::default() })
}
