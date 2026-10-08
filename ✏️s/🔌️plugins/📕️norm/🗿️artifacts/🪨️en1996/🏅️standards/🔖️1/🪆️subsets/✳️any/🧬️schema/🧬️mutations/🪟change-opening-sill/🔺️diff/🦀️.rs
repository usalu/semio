use super::ChangeOpeningSill;
use crate::{En1996Diff, En1996Snapshot};
use crate::diff::{En1996OpeningDelta, En1996OpeningPatch, En1996WallDelta, En1996WallPatch};
pub fn diff(payload: &ChangeOpeningSill, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].openings.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid opening index."), Vec::<String>::new());
    }
    let wall = &base.walls[payload.wall_index];
    let opening = &wall.openings[payload.index];
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&wall.id, En1996WallPatch { openings: En1996OpeningDelta::modification(&opening.id, En1996OpeningPatch { sill_height_m: Some(payload.new_sill_height_m), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
