//! 🔺️ Diff for `insert-slab`.
use crate::insert_slab::InsertSlab;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994SlabsRows, En1994SlabsInserted};

pub fn diff(payload: &InsertSlab, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.slabs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    if base.slabs.iter().any(|row| row.id == payload.slab.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A slab with id '{}' already exists.", payload.slab.id), [payload.slab.id.clone()]);
    }
    protocol::MutationOutcome::new(En1994Diff { slabs: Some(En1994SlabsRows::insertion(payload.index, payload.slab.clone())), ..Default::default() })
}
