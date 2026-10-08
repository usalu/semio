use crate::diff::{En1992Diff, En1992ConcreteGradesRows, En1992ConcreteGradesPatch};
use super::ChangeConcreteFCk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeConcreteFCk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(g) = base.concrete_grades.iter().find(|g| g.id == payload.grade_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grade {} not found.", payload.grade_id), Vec::<String>::new());
    };
    if (g.f_ck - payload.new_f_ck).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        concrete_grades: Some(En1992ConcreteGradesRows::modification(&payload.grade_id, En1992ConcreteGradesPatch { f_ck: Some(payload.new_f_ck), ..Default::default() })),
        ..Default::default()
    })
}
