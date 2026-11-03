//! 🧱 `remove-concrete-grade` diff: removes the addressed concrete grade row.

use crate::remove_concrete_grade::RemoveConcreteGrade;
use semio_s_artifact_norm_en1992_diff::{En1992ConcreteGradesRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveConcreteGrade, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    let index = payload.index;
    if index >= base.concrete_grades.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "concrete_grades index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { concrete_grades: Some(En1992ConcreteGradesRows::removal(&base.concrete_grades, index)), ..Default::default() })
}
