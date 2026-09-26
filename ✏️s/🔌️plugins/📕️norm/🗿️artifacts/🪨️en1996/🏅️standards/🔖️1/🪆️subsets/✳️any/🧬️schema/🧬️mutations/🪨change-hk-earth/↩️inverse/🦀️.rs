use super::ChangeHKEarth;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeHKEarth, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeHKEarth(ChangeHKEarth {
            wall_index: payload.wall_index,
            index: payload.index,
            new_h_k_earth_n: base.walls[payload.wall_index].load_cases[payload.index].h_k_earth_n,
        })]
    }
}
