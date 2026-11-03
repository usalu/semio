//! 🔺️ `change-wall-rebar-friction` diff: patches `reinforcement_friction` of the addressed retaining wall row.

use crate::change_wall_rebar_friction::ChangeWallRebarFriction;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsPatch, En1997RetainingWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWallRebarFriction, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_reinforcement_friction.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.reinforcement_friction == payload.new_reinforcement_friction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::modification(&payload.id, En1997RetainingWallsPatch { reinforcement_friction: Some(payload.new_reinforcement_friction), ..Default::default() })), ..Default::default() })
}
