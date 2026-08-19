//! 🔺️ Sparse diff construction for the `replace-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::ReplaceSiteContext;
use crate::artifacts::program::diff::{ProgramSiteContextDelta, ProgramSiteContextPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No site context exists with this id.", [payload.site_context.header.id.0.clone()]);
    };
    if existing == &payload.site_context {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This site context already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.site_context).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.site_context.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
