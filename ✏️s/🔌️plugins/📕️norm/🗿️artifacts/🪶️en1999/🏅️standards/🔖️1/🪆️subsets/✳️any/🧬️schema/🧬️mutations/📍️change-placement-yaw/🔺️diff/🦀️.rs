//! 🔺️ `change-placement-yaw` diff: patches `yaw` of the addressed placement row.

use crate::change_placement_yaw::ChangePlacementYaw;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999PlacementsPatch, En1999PlacementsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangePlacementYaw, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_yaw.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "placement", Vec::<String>::new());
    };
    if r0.yaw == payload.new_yaw {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { placements: Some(En1999PlacementsRows::modification(&payload.id, En1999PlacementsPatch { yaw: Some(payload.new_yaw), ..Default::default() })), ..Default::default() })
}
