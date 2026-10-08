use super::InsertLoadCase;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996LoadCaseDelta, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &InsertLoadCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index > base.walls[payload.wall_index].load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case insert."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    if wall.load_cases.iter().any(|existing| existing.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { load_cases: En1996LoadCaseDelta::insertion(&wall.load_cases, payload.index, payload.load_case.clone()), ..Default::default() }), ..Default::default() })
}
