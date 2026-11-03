//! 📍 `change-placement-x` diff: patches `x` of the addressed placement row.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PlacementDelta, En1990PlacementPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangePlacementX, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { placements: En1990PlacementDelta::modification(&payload.id, En1990PlacementPatch { x: Some(payload.new_x), ..Default::default() }), ..Default::default() })
}
