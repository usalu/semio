//! 🔺️ `change-placement-y` diff: patches `y` of the addressed placement row.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlacementDelta, En1993PlacementPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePlacementY, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "placement", Vec::<String>::new());
    };
    if row.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { placements: En1993PlacementDelta::modification(payload.id.clone(), En1993PlacementPatch { y: Some(payload.new_y), ..Default::default() }), ..Default::default() })
}
