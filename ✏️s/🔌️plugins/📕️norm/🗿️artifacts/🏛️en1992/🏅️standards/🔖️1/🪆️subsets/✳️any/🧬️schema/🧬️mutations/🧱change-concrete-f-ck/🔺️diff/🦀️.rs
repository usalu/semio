use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992ConcreteGradesRows, En1992ConcreteGradesPatch};
use crate::change_concrete_f_ck::ChangeConcreteFCk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

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
