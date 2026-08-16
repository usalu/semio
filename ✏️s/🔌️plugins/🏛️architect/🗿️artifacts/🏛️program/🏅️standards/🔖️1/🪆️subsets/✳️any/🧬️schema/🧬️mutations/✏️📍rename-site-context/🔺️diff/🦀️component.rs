//! 🔺️ Sparse diff construction for the `rename-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::RenameSiteContext;
use crate::artifacts::program::diff::{ProgramSiteContextDelta, ProgramSiteContextPatchEntry};
use crate::artifacts::program::registers::SiteContextPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.site_context.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No site context exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This site context already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = SiteContextPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
