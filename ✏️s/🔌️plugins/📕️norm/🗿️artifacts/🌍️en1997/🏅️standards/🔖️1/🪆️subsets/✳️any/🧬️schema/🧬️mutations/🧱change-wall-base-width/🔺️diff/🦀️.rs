use super::ChangeWallBaseWidth;
use crate::diff::{En1997Diff, En1997WallList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeWallBaseWidth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_base_width.is_finite() || payload.new_base_width <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "base width must be positive", vec![payload.id.clone()]);
    }
    let Some(idx) = base.retaining_walls.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut retaining_walls = base.retaining_walls.clone();
    retaining_walls[idx].base_width = payload.new_base_width;
    protocol::MutationOutcome::new(En1997Diff { retaining_walls: Some(En1997WallList { values: retaining_walls }), ..Default::default() })
}
