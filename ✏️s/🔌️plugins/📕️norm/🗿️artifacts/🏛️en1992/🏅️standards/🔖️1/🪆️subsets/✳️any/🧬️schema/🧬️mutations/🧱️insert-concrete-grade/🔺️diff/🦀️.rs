//! 🧱 `insert-concrete-grade` diff: inserts the concrete grade row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_concrete_grade::InsertConcreteGrade;
use semio_s_artifact_norm_en1992_diff::{En1992ConcreteGradesRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertConcreteGrade, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    if base.concrete_grades.iter().any(|existing| existing.id == payload.concrete_grade.id) {
        let key = payload.concrete_grade.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Concrete grade {key} already exists."), [key]);
    }
    if payload.index > base.concrete_grades.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.concrete_grades.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { concrete_grades: Some(En1992ConcreteGradesRows::insertion(payload.index, payload.concrete_grade.clone())), ..Default::default() })
}
