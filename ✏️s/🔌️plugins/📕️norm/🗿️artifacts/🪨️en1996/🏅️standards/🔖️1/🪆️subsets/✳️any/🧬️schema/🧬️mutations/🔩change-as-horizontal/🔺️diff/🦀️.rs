use super::ChangeAsHorizontal;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeAsHorizontal, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid wall index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { as_horizontal_m2: Some(payload.new_as_horizontal_m2), ..Default::default() }), ..Default::default() })
}
