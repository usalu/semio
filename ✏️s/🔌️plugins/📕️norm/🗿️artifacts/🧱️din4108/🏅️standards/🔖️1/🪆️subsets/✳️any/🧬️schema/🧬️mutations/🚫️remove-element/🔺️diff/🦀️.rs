//! 🚫️ `remove-element` diff — removes the row at the index, guarded by the row's own id; an index past the list's end is a `mutation.invariant`.

use super::RemoveElement;
use crate::diff::Din4108RowEdit as _;
use crate::diff::{Din4108Diff, Din4108ElementEdit};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveElement, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    let Some(row) = base.elements.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element index out of range", Vec::<String>::new());
    };
    protocol::MutationOutcome::new(Din4108Diff { elements: vec![Din4108ElementEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
