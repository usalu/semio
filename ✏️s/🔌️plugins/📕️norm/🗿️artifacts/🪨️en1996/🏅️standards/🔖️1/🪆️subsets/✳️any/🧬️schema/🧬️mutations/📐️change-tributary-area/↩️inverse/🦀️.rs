use super::ChangeTributaryArea;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeTributaryArea, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeTributaryArea(ChangeTributaryArea {
            wall_index: payload.wall_index,
            index: payload.index,
            new_tributary_area_m2: base.walls[payload.wall_index].load_cases[payload.index].tributary_area_m2,
        })]
    }
}
