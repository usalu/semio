//! 📍️ `change-placement-x` diff: patches `x` of the addressed placement row.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PlacementsPatch, En1997PlacementsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePlacementX, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { placements: Some(En1997PlacementsRows::modification(&payload.id, En1997PlacementsPatch { x: Some(payload.new_x), ..Default::default() })), ..Default::default() })
}
