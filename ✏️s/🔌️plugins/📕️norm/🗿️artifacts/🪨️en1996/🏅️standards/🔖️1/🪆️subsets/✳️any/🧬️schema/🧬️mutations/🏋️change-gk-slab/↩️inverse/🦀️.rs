use super::ChangeGKSlab;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeGKSlab, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeGKSlab(ChangeGKSlab {
            wall_index: payload.wall_index,
            index: payload.index,
            new_g_k_slab_n: base.walls[payload.wall_index].load_cases[payload.index].g_k_slab_n,
        })]
    }
}
