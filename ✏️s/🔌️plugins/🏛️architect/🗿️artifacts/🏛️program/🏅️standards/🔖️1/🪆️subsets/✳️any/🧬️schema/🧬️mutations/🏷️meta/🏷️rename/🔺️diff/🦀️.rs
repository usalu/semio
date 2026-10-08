//! 🔺️ Sparse diff construction for the `rename-meta` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏷️update-meta` per Wave C.

use super::RenameMeta;
use crate::diff::ProgramMetaEdit;
use crate::registers::ProgramMetaPatch;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✏️ Patches only `title` of the section. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the title is unchanged.
pub fn diff(payload: &RenameMeta, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.meta.title == payload.new_title {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "Document metadata already has this title.").at([base.meta.document_id.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { meta: Some(ProgramMetaEdit::patching(ProgramMetaPatch { title: Some(payload.new_title.clone()), ..Default::default() })), ..Default::default() })
}
