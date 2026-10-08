use super::InsertWall;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996WallDelta};
pub fn diff(payload: &InsertWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index > base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid insert index."), Vec::<String>::new());
    }
    if base.walls.iter().any(|existing| existing.id == payload.wall.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.wall.id), [payload.wall.id.clone()]);
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::insertion(payload.index, payload.wall.clone()), ..Default::default() })
}
