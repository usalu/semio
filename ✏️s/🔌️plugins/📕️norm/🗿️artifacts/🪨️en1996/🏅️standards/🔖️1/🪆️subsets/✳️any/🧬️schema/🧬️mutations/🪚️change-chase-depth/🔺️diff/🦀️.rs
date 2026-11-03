//! 🪚️ `change-chase-depth` diff: patches `depth_m` of the addressed chase row.

use crate::change_chase_depth::ChangeChaseDepth;
use semio_s_artifact_norm_en1996_diff::{En1996ChaseDelta, En1996ChasePatch, En1996Diff};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeChaseDepth, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_depth_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.chases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Chase {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.depth_m == payload.new_depth_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The chase already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { chases: En1996ChaseDelta::modification(&payload.id, En1996ChasePatch { depth_m: Some(payload.new_depth_m), ..Default::default() }), ..Default::default() })
}
