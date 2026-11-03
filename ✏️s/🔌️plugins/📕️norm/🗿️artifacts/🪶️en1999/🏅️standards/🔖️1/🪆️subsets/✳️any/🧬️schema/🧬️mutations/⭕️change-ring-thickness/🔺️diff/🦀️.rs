//! 🔺️ `change-ring-thickness` diff: patches `thickness` of the addressed ring stiffener row.

use crate::change_ring_thickness::ChangeRingThickness;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999RingStiffenersPatch, En1999RingStiffenersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeRingThickness, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "ring-stiffener", Vec::<String>::new());
    };
    if r0.thickness == payload.new_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The ring stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { ring_stiffeners: Some(En1999RingStiffenersRows::modification(&payload.ring_stiffener_id, En1999RingStiffenersPatch { thickness: Some(payload.new_thickness), ..Default::default() })), ..Default::default() })
}
