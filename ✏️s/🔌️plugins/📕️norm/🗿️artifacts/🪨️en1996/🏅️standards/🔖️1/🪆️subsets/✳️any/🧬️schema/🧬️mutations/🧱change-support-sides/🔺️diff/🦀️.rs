use super::ChangeSupportSides;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeSupportSides, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid wall index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { support_sides: Some(payload.new_support_sides), ..Default::default() }), ..Default::default() })
}
