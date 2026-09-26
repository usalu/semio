//! 🔺️ `change-project-id` sparse diff.

use super::ChangeProjectId;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeProjectId, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.project_id == &mutation.new_project_id {
        return MutationOutcome::empty().warn("mutation.no-op", "project_id already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        project_id: Some(mutation.new_project_id.clone()),
        ..En1990Diff::default()
    })
}
