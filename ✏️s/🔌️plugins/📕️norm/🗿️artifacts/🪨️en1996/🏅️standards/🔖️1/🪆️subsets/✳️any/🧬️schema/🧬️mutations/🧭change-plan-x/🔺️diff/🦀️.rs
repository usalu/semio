//! 🧭 `change-plan-x` diff: patches `plan_x_m` of the addressed wall row.

use crate::change_plan_x::ChangePlanX;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangePlanX, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_plan_x_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.plan_x_m == payload.new_plan_x_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { plan_x_m: Some(payload.new_plan_x_m), ..Default::default() }), ..Default::default() })
}
