use super::RemoveLoadCase;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &RemoveLoadCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = {
        if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
            return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case remove."), Vec::<String>::new());
        }
        let mut walls = base.walls.clone();
        walls[payload.wall_index].load_cases.remove(payload.index);
        En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() }
    };
    protocol::MutationOutcome::new(diff)
}
