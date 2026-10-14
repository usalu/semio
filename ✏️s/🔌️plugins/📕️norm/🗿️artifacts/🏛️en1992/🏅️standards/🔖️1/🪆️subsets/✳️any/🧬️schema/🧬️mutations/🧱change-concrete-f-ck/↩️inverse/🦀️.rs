use crate::change_concrete_f_ck::ChangeConcreteFCk;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeConcreteFCk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concrete_f_ck::ChangeConcreteFCk> {
    Ok((|| {
    let Some(g) = base.concrete_grades.iter().find(|g| g.id == payload.grade_id) else { return vec![]; };
    vec![Op::from(ChangeConcreteFCk { grade_id: payload.grade_id.clone(), new_f_ck: g.f_ck })]

    })())
}
