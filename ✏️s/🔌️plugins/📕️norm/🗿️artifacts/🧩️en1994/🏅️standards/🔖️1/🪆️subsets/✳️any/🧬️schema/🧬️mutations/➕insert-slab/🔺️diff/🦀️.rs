//! Diff for `insert-slab`.
use super::InsertSlab;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994SlabsRows, En1994SlabsInserted};

pub fn diff(payload: &InsertSlab, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.slabs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabsRows { inserted: vec![En1994SlabsInserted { index: payload.index, row: payload.slab.clone() }], ..Default::default() }), ..Default::default() })
}
