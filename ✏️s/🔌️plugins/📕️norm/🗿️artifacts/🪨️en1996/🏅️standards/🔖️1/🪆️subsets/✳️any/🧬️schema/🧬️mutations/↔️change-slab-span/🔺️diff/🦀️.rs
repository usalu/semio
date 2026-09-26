use super::ChangeSlabSpan;
use crate::diff::En1996WallList;
use crate::{En1996Diff, En1996Snapshot};
pub fn diff(payload: &ChangeSlabSpan, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", String::from("Invalid load-case index."), Vec::<String>::new());
    }
    let mut walls = base.walls.clone();
    walls[payload.wall_index].load_cases[payload.index].slab_span_m = payload.new_slab_span_m;
    protocol::MutationOutcome::new(En1996Diff { walls: Some(En1996WallList { values: walls }), ..Default::default() })
}
