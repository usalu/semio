//! 🔺️ `change-wall-reinforcement-width` diff: patches `reinforcement_width` of the addressed retaining wall row.

use crate::change_wall_reinforcement_width::ChangeWallReinforcementWidth;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997RetainingWallsPatch, En1997RetainingWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWallReinforcementWidth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_reinforcement_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.reinforcement_width == payload.new_reinforcement_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997RetainingWallsRows::modification(&payload.id, En1997RetainingWallsPatch { reinforcement_width: Some(payload.new_reinforcement_width), ..Default::default() })), ..Default::default() })
}
