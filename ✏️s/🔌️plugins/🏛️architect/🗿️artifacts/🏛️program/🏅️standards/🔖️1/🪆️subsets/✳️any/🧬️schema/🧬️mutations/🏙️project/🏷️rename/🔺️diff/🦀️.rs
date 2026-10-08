//! 🔺️ Sparse diff construction for the `rename-project` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📁update-project` per Wave C.

use super::RenameProject;
use crate::diff::ProjectDefinitionEdit;
use crate::registers::ProjectDefinitionPatch;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✏️ Patches only `code` of the section. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the code is unchanged.
pub fn diff(payload: &RenameProject, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.project.code == payload.new_code {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "Project already has this code.").at([base.project.id.0.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { project: Some(ProjectDefinitionEdit::patching(ProjectDefinitionPatch { code: Some(payload.new_code.clone()), ..Default::default() })), ..Default::default() })
}
