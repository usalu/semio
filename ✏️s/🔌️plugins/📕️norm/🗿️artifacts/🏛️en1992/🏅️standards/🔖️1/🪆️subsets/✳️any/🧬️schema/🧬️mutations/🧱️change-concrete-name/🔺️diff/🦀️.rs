//! 🧱 `change-concrete-name` diff: patches `name` of the addressed concrete grade row.

use crate::change_concrete_name::ChangeConcreteName;
use semio_s_artifact_norm_en1992_diff::{En1992ConcreteGradesPatch, En1992ConcreteGradesRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeConcreteName, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.concrete_grades.iter().find(|row| row.id == payload.grade_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Concrete grade {} does not exist.", payload.grade_id), [payload.grade_id.clone()]);
    };
    if r0.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The concrete grade already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { concrete_grades: Some(En1992ConcreteGradesRows::modification(&payload.grade_id, En1992ConcreteGradesPatch { name: Some(payload.new_name.clone()), ..Default::default() })), ..Default::default() })
}
