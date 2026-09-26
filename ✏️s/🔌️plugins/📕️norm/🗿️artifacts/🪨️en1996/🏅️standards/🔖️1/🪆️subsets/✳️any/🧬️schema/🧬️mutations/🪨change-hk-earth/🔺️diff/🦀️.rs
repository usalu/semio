use super::ChangeHKEarth;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeHKEarth, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case index."), Vec::<String>::new());
    }
    let mut walls = base.walls.clone();
    walls[payload.wall_index].load_cases[payload.index].h_k_earth_n = payload.new_h_k_earth_n;
    protocol::MutationOutcome::new(En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() })
}
