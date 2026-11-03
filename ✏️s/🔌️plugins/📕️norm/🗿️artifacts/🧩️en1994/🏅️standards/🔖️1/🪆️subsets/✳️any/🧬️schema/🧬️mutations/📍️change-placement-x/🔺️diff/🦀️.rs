//! 📍️ `change-placement-x` diff on the positional `placements` lane.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994PlacementsPatch, En1994PlacementsRows};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &ChangePlacementX, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if base.placements[index].x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows::modification(&payload.id, En1994PlacementsPatch { x: Some(payload.new_x), ..Default::default() })), ..Default::default() })
}
