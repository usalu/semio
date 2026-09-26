use super::ChangeDensity;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeDensity, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.index >= base.walls.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid wall index."), Vec::<String>::new());
    }
    let mut walls = base.walls.clone();
    walls[payload.index].density_kg_m3 = payload.new_density_kg_m3;
    protocol::MutationOutcome::new(En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() })
}
