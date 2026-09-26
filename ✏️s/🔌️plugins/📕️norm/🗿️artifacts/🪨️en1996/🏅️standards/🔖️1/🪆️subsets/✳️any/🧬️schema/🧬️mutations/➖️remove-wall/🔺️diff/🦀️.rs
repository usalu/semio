use super::RemoveWall;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &RemoveWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = {
        if payload.index >= base.walls.len() {
            return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid remove index."), Vec::<String>::new());
        }
        let mut walls = base.walls.clone();
        walls.remove(payload.index);
        En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() }
    };
    protocol::MutationOutcome::new(diff)
}
