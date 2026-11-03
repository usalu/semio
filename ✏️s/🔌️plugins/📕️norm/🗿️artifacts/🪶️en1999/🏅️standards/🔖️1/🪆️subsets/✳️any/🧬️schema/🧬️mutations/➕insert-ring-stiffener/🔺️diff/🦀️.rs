//! 🔺️ `insert-ring-stiffener` diff.

use crate::insert_ring_stiffener::InsertRingStiffener;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999RingStiffenersRows};

pub fn diff(payload: &InsertRingStiffener, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.ring_stiffeners.iter().any(|existing| existing.id == payload.ring_stiffener.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Ring stiffener id {} already exists.", payload.ring_stiffener.id), [payload.ring_stiffener.id.clone()]);
    }
    if (payload.index as usize) > base.ring_stiffeners.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.ring_stiffeners.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { ring_stiffeners: Some(En1999RingStiffenersRows::insertion(payload.index as usize, payload.ring_stiffener.clone())), ..Default::default() })
}
