use super::RemoveLoadCase;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996LoadCaseDelta, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &RemoveLoadCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case remove."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { load_cases: En1996LoadCaseDelta::removal(&wall.load_cases, payload.index), ..Default::default() }), ..Default::default() })
}
