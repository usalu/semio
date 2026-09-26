use crate::diff::{En1992Diff, En1992ReinforcementGradeList};
use crate::mutations::change_reinforcement_f_yk::ChangeReinforcementFYk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeReinforcementFYk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut grades = base.reinforcement_grades.clone();
    let Some(g) = grades.iter_mut().find(|g| g.id == payload.grade_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Grade {} not found.", payload.grade_id), Vec::<String>::new());
    };
    if (g.f_yk - payload.new_f_yk).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    g.f_yk = payload.new_f_yk;
    protocol::MutationOutcome::new(En1992Diff { reinforcement_grades: Some(En1992ReinforcementGradeList { values: grades }), ..Default::default() })
}
