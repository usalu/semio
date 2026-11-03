//! 📍️ `change-placement-y` diff: patches `y` of the addressed placement row.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996PlacementDelta, En1996PlacementPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangePlacementY, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { placements: En1996PlacementDelta::modification(&payload.id, En1996PlacementPatch { y: Some(payload.new_y), ..Default::default() }), ..Default::default() })
}
