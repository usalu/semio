use crate::diff::{En1992Diff, En1992ConcreteGradeList};
use crate::mutations::change_concrete_f_ck::ChangeConcreteFCk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeConcreteFCk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut grades = base.concrete_grades.clone();
    let Some(g) = grades.iter_mut().find(|g| g.id == payload.grade_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Grade {} not found.", payload.grade_id), Vec::<String>::new());
    };
    if (g.f_ck - payload.new_f_ck).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    g.f_ck = payload.new_f_ck;
    protocol::MutationOutcome::new(En1992Diff { concrete_grades: Some(En1992ConcreteGradeList { values: grades }), ..Default::default() })
}
