//! 🚫️ `remove-element` diff — removes the row at the index; an index past the list's end is a `mutation.invariant`.

use super::RemoveElement;
use crate::diff::{Din4108Diff, Din4108ElementDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &RemoveElement, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if payload.index >= base.elements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "element index out of range", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::removal(&base.elements, payload.index), ..Default::default() })
}
