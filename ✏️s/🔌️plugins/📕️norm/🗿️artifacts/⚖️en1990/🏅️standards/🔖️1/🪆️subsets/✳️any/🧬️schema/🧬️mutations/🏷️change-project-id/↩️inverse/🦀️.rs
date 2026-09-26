//! ↩️ `change-project-id` inverse.

use super::ChangeProjectId;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeProjectId, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeProjectId(ChangeProjectId { new_project_id: base.project_id.clone() })]
}
