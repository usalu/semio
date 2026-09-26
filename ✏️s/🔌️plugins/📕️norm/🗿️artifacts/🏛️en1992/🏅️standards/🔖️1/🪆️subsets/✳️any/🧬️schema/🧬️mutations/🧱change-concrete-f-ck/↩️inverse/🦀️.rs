use crate::mutations::change_concrete_f_ck::ChangeConcreteFCk;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeConcreteFCk, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(g) = base.concrete_grades.iter().find(|g| g.id == payload.grade_id) else { return vec![]; };
    vec![En1992Mutation::ChangeConcreteFCk(ChangeConcreteFCk { grade_id: payload.grade_id.clone(), new_f_ck: g.f_ck })]
}
