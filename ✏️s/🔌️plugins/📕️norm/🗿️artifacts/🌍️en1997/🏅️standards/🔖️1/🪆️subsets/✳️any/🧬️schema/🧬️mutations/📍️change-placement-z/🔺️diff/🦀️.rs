//! 📍️ `change-placement-z` diff: patches `z` of the addressed placement row.

use crate::change_placement_z::ChangePlacementZ;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PlacementsPatch, En1997PlacementsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePlacementZ, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.placements.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { placements: Some(En1997PlacementsRows::modification(&payload.id, En1997PlacementsPatch { z: Some(payload.new_z), ..Default::default() })), ..Default::default() })
}
