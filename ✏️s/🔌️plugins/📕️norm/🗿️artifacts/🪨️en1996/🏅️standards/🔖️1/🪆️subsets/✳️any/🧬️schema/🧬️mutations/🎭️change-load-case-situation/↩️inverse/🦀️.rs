use super::ChangeLoadCaseSituation;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeLoadCaseSituation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![En1996Mutation::ChangeLoadCaseSituation(ChangeLoadCaseSituation {
            wall_index: payload.wall_index,
            load_case_index: payload.load_case_index,
            new_design_situation: base.walls[payload.wall_index].load_cases[payload.load_case_index].design_situation.clone(),
        })]
    }
}
