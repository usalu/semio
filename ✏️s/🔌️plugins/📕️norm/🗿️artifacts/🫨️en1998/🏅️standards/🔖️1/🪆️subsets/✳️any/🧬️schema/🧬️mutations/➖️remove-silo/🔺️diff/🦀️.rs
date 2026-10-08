//! ➖️ `remove-silo` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveSilo;
use crate::diff::{En1998Diff, En1998SiloDelta};
use crate::En1998Snapshot;

pub fn diff(payload: &RemoveSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.silos.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::removal(&base.silos, payload.index), ..Default::default() })
}
