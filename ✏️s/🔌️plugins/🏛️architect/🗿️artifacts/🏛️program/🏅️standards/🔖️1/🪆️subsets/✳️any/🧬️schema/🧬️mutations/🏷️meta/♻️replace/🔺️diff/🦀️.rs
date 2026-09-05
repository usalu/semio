//! 🔺️ Sparse diff construction for the `replace-meta` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏷️update-meta` per Wave C.

use super::ReplaceMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `ProgramMeta` wholesale. Root-scoped singleton — always present, so Warning
/// `mutation.no-op` (empty diff) covers the only degenerate case: the value is unchanged.
pub async fn diff(payload: &ReplaceMeta, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.meta == payload.new_meta {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Document metadata already matches the requested value.").at([base.meta.document_id.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { meta: Some(payload.new_meta.clone()), ..Default::default() })
}
