use crate::change_reinforcement_f_yk::ChangeReinforcementFYk;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementFYk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_f_yk::ChangeReinforcementFYk> {
    Ok((|| {
    let Some(g) = base.reinforcement_grades.iter().find(|g| g.id == payload.grade_id) else { return vec![]; };
    vec![Op::from(ChangeReinforcementFYk { grade_id: payload.grade_id.clone(), new_f_yk: g.f_yk })]

    })())
}
