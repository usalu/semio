//! 🔺️ `change-project-id` sparse diff.

use crate::change_project_id::ChangeProjectId;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeProjectId, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.project_id == &mutation.new_project_id {
        return MutationOutcome::empty().warning("mutation.no-op", "project_id already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        project_id: Some(mutation.new_project_id.clone()),
        ..En1990Diff::default()
    })
}
