//! 🔺️ Sparse diff construction for the `create-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::CreateSiteContext;
use crate::diff::ProgramSiteContextDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.site_context.header.id;
    if base.site_context.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A site context already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.site_context.len());
    if at > base.site_context.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the site context list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(ProgramSiteContextDelta::insertion(at, payload.site_context.clone())), ..Default::default() })
}
