//! 🔺️ `remove-ring-stiffener` diff.

use crate::remove_ring_stiffener::RemoveRingStiffener;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999RingStiffenersRows};

pub fn diff(payload: &RemoveRingStiffener, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.ring_stiffeners.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown ring stiffener id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { ring_stiffeners: Some(En1999RingStiffenersRows::removal(&base.ring_stiffeners, index)), ..Default::default() })
}
