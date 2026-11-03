//! ↗️ `change-eccentricity-bottom` diff: patches `eccentricity_bottom_m` of the addressed wall row.

use crate::change_eccentricity_bottom::ChangeEccentricityBottom;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_eccentricity_bottom_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.eccentricity_bottom_m == payload.new_eccentricity_bottom_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { eccentricity_bottom_m: Some(payload.new_eccentricity_bottom_m), ..Default::default() }), ..Default::default() })
}
