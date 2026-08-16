//! 🔺️ Sparse diff construction for the `delete-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::DeleteSiteContext;
use crate::artifacts::program::diff::ProgramSiteContextDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.site_context.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No site context exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(ProgramSiteContextDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
