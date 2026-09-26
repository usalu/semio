use super::InsertConcentrated;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &InsertConcentrated, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = {
        if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index > base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() {
            return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid concentrated insert."), Vec::<String>::new());
        }
        let mut walls = base.walls.clone();
        walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.insert(payload.index, payload.load.clone());
        En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() }
    };
    protocol::MutationOutcome::new(diff)
}
