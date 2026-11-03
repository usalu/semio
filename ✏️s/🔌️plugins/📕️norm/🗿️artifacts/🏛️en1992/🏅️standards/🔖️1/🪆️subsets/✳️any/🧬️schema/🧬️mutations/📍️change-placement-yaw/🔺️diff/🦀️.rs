//! 📍️ `change-placement-yaw` diff: patches `yaw` of the addressed placement row.

use crate::change_placement_yaw::ChangePlacementYaw;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PlacementsPatch, En1992PlacementsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangePlacementYaw, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_yaw.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.yaw == payload.new_yaw {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { placements: Some(En1992PlacementsRows::modification(&payload.id, En1992PlacementsPatch { yaw: Some(payload.new_yaw), ..Default::default() })), ..Default::default() })
}
