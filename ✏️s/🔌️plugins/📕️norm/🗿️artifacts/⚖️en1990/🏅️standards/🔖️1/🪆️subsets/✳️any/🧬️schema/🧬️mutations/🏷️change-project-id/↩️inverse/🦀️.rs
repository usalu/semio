//! ↩️ `change-project-id` inverse.

use crate::change_project_id::ChangeProjectId;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeProjectId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_project_id::ChangeProjectId> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeProjectId { new_project_id: base.project_id.clone() })]

    })())
}
