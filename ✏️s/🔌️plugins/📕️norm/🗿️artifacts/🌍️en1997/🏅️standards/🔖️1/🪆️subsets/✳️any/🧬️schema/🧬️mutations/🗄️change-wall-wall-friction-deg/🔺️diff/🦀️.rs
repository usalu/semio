//! 🔺️ `change-wall-wall-friction-deg` diff: patches `wall_friction_deg` of the addressed retaining wall row.

use crate::change_wall_wall_friction_deg::ChangeWallWallFrictionDeg;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsPatch, En1997RetainingWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWallWallFrictionDeg, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_wall_friction_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.wall_friction_deg == payload.new_wall_friction_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::modification(&payload.id, En1997RetainingWallsPatch { wall_friction_deg: Some(payload.new_wall_friction_deg), ..Default::default() })), ..Default::default() })
}
