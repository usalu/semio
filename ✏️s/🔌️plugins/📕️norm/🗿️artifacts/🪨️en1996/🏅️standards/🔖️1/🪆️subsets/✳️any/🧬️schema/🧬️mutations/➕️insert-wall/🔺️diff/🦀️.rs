use super::InsertWall;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &InsertWall, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = {
        if payload.index > base.walls.len() {
            return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid insert index."), Vec::<String>::new());
        }
        let mut walls = base.walls.clone();
        walls.insert(payload.index, payload.wall.clone());
        En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() }
    };
    protocol::MutationOutcome::new(diff)
}
