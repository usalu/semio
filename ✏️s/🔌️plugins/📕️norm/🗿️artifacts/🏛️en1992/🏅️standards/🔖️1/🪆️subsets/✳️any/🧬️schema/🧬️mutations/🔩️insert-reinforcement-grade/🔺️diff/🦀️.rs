//! 🔩 `insert-reinforcement-grade` diff: inserts the reinforcement grade row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_reinforcement_grade::InsertReinforcementGrade;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992ReinforcementGradesRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertReinforcementGrade, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    if base.reinforcement_grades.iter().any(|existing| existing.id == payload.reinforcement_grade.id) {
        let key = payload.reinforcement_grade.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Reinforcement grade {key} already exists."), [key]);
    }
    if payload.index > base.reinforcement_grades.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.reinforcement_grades.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { reinforcement_grades: Some(En1992ReinforcementGradesRows::insertion(payload.index, payload.reinforcement_grade.clone())), ..Default::default() })
}
