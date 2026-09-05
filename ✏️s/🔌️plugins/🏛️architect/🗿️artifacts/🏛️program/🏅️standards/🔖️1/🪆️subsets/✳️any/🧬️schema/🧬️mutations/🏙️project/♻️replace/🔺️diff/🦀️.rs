//! 🔺️ Sparse diff construction for the `replace-project` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📁update-project` per Wave C.

use super::ReplaceProject;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `ProjectDefinition` wholesale. Root-scoped singleton — always present, so Warning
/// `mutation.no-op` (empty diff) covers the only degenerate case: the value is unchanged.
pub async fn diff(payload: &ReplaceProject, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.project == payload.new_project {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Project already matches the requested value.").at([base.project.id.0.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { project: Some(payload.new_project.clone()), ..Default::default() })
}
