use crate::mutations::change_reinforcement_f_yk::ChangeReinforcementFYk;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeReinforcementFYk, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(g) = base.reinforcement_grades.iter().find(|g| g.id == payload.grade_id) else { return vec![]; };
    vec![En1992Mutation::ChangeReinforcementFYk(ChangeReinforcementFYk { grade_id: payload.grade_id.clone(), new_f_yk: g.f_yk })]
}
