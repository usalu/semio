//! 🔺️ `change-placement-y` diff: patches `y` of the addressed placement row.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PlacementsPatch, En1999PlacementsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangePlacementY, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "placement", Vec::<String>::new());
    };
    if r0.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { placements: Some(En1999PlacementsRows::modification(&payload.id, En1999PlacementsPatch { y: Some(payload.new_y), ..Default::default() })), ..Default::default() })
}
