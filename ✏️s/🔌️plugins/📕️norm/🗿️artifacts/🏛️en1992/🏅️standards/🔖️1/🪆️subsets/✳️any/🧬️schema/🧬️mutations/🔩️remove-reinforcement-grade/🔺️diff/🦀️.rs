//! 🔩 `remove-reinforcement-grade` diff: removes the addressed reinforcement grade row.

use crate::remove_reinforcement_grade::RemoveReinforcementGrade;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992ReinforcementGradesRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveReinforcementGrade, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    let index = payload.index;
    if index >= base.reinforcement_grades.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "reinforcement_grades index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { reinforcement_grades: Some(En1992ReinforcementGradesRows::removal(&base.reinforcement_grades, index)), ..Default::default() })
}
