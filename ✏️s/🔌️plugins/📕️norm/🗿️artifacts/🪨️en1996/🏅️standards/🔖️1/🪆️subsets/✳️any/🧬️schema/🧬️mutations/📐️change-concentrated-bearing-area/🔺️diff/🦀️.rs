use super::ChangeConcentratedBearingArea;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeConcentratedBearingArea, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let diff = {
        if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() {
            return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid concentrated index."), Vec::<String>::new());
        }
        let mut walls = base.walls.clone();
        walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index].bearing_area_m2 = payload.new_bearing_area_m2;
        En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() }
    };
    protocol::MutationOutcome::new(diff)
}
