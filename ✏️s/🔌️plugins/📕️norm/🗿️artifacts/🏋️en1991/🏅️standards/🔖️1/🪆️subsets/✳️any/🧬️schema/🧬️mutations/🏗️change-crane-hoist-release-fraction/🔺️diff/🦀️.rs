//! 🏗 `change-crane-hoist-release-fraction` diff: patches `release_fraction` of the addressed crane hoist row.

use crate::change_crane_hoist_release_fraction::ChangeCraneHoistReleaseFraction;
use semio_s_artifact_norm_en1991_diff::{En1991CraneHoistDelta, En1991CraneHoistPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeCraneHoistReleaseFraction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_release_fraction.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.crane_hoists.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Crane hoist {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.release_fraction == payload.new_release_fraction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane hoist already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_hoists: En1991CraneHoistDelta::modification(&payload.id, En1991CraneHoistPatch { release_fraction: Some(payload.new_release_fraction), ..Default::default() }), ..Default::default() })
}
