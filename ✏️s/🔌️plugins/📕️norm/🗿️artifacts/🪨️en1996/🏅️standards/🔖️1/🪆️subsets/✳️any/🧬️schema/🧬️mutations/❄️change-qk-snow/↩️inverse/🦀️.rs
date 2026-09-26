use super::ChangeQKSnow;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeQKSnow, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeQKSnow(ChangeQKSnow {
            wall_index: payload.wall_index,
            index: payload.index,
            new_q_k_snow_pa: base.walls[payload.wall_index].load_cases[payload.index].q_k_snow_pa,
        })]
    }
}
