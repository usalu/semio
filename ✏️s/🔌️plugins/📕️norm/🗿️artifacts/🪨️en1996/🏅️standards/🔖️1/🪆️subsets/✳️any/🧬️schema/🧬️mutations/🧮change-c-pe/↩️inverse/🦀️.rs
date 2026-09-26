use super::ChangeCPe;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeCPe, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeCPe(ChangeCPe {
            wall_index: payload.wall_index,
            index: payload.index,
            new_c_pe: base.walls[payload.wall_index].load_cases[payload.index].c_pe,
        })]
    }
}
