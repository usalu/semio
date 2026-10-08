use crate::diff::{En1992Diff, En1992ReinforcementGradesRows, En1992ReinforcementGradesPatch};
use super::ChangeReinforcementFYk;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeReinforcementFYk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(g) = base.reinforcement_grades.iter().find(|g| g.id == payload.grade_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grade {} not found.", payload.grade_id), Vec::<String>::new());
    };
    if (g.f_yk - payload.new_f_yk).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        reinforcement_grades: Some(En1992ReinforcementGradesRows::modification(&payload.grade_id, En1992ReinforcementGradesPatch { f_yk: Some(payload.new_f_yk), ..Default::default() })),
        ..Default::default()
    })
}
