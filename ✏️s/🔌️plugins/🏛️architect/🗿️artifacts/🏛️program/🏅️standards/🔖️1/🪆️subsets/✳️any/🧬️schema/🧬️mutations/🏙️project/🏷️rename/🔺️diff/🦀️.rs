//! 🔺️ Sparse diff construction for the `rename-project` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📁update-project` per Wave C.

use super::RenameProject;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProjectDefinition` with only `code` changed. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the code is unchanged.
pub async fn diff(payload: &RenameProject, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.project.code == payload.new_code {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Project already has this code.").at([base.project.id.0.clone()])]);
    }
    let mut value = base.project.clone();
    value.code = payload.new_code.clone();
    protocol::MutationOutcome::new(ProgramDiff { project: Some(value), ..Default::default() })
}
