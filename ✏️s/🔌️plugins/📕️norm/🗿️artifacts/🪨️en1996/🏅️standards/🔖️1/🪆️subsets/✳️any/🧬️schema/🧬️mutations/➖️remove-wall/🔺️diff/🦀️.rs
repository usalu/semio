use super::RemoveWall;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996WallDelta};
pub fn diff(payload: &RemoveWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid remove index."), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::removal(&base.walls, payload.index), ..Default::default() })
}
