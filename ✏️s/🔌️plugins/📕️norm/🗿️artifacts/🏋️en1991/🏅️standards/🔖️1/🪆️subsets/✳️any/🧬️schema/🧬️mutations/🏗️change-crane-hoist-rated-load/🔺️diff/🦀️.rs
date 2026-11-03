//! 🏗 `change-crane-hoist-rated-load` diff: patches `rated_load` of the addressed crane hoist row.

use crate::change_crane_hoist_rated_load::ChangeCraneHoistRatedLoad;
use semio_s_artifact_norm_en1991_diff::{En1991CraneHoistDelta, En1991CraneHoistPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeCraneHoistRatedLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_rated_load.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.crane_hoists.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Crane hoist {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.rated_load == payload.new_rated_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane hoist already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_hoists: En1991CraneHoistDelta::modification(&payload.id, En1991CraneHoistPatch { rated_load: Some(payload.new_rated_load), ..Default::default() }), ..Default::default() })
}
