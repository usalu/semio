use super::InsertOpening;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996OpeningDelta, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &InsertOpening, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index > base.walls[payload.wall_index].openings.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid opening insert."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    if wall.openings.iter().any(|existing| existing.id == payload.opening.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.opening.id), [payload.opening.id.clone()]);
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { openings: En1996OpeningDelta::insertion(payload.index, payload.opening.clone()), ..Default::default() }), ..Default::default() })
}
