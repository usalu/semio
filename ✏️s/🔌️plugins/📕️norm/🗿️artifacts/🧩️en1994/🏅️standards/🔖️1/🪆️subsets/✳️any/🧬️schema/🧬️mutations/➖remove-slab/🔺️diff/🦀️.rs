//! Diff for `remove-slab`.
use super::RemoveSlab;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994SlabsRows};

pub fn diff(payload: &RemoveSlab, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.slabs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabsRows { removed: vec![payload.index], ..Default::default() }), ..Default::default() })
}
