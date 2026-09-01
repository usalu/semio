//! 🔺️ Sparse diff construction for the `rename-meta` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏷️update-meta` per Wave C.

use super::RenameMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProgramMeta` with only `title` changed. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the title is unchanged.
pub async fn diff(payload: &RenameMeta, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.meta.title == payload.new_title {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Document metadata already has this title.").at([base.meta.document_id.clone()])]);
    }
    let mut value = base.meta.clone();
    value.title = payload.new_title.clone();
    protocol::MutationOutcome::new(ProgramDiff { meta: Some(value), ..Default::default() })
}
