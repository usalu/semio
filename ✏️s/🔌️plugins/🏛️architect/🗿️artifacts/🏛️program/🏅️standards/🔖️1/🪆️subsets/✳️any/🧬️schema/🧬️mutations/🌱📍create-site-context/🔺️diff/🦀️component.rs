//! 🔺️ Sparse diff construction for the `create-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::CreateSiteContext;
use crate::artifacts::program::diff::ProgramSiteContextDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.site_context.header.id.clone();
    if base.site_context.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A site context already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(ProgramSiteContextDelta { added: vec![payload.site_context.clone()], ..Default::default() }), ..Default::default() })
}
