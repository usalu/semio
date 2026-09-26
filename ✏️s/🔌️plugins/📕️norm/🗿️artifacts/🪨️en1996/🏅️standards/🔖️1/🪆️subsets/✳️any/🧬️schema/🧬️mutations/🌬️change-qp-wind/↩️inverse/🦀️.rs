use super::ChangeQPWind;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeQPWind, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeQPWind(ChangeQPWind {
            wall_index: payload.wall_index,
            index: payload.index,
            new_q_p_wind_pa: base.walls[payload.wall_index].load_cases[payload.index].q_p_wind_pa,
        })]
    }
}
