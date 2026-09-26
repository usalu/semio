//! Diff for `insert-slab`.
use super::InsertSlab;
use crate::diff::En1994SlabList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &InsertSlab, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.slabs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    let mut slabs = base.slabs.clone();
    slabs.insert(payload.index, payload.slab.clone());
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabList { values: slabs }), ..Default::default() })
}
