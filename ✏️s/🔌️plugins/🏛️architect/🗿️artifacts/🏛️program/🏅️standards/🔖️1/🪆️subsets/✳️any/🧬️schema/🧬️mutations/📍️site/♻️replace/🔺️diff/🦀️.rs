//! 🔺️ Sparse diff construction for the `replace-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::ReplaceSiteContext;
use crate::diff::ProgramSiteContextDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceSiteContext, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.site_context.header.id;
    let Some(position) = base.site_context.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No site context exists with this id.", [id.0.clone()]);
    };
    if base.site_context[position] == payload.site_context {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This site context already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramSiteContextDelta::removal(&base.site_context, position);
    delta.absorb(ProgramSiteContextDelta::insertion(position, payload.site_context.clone()));
    protocol::MutationOutcome::new(ProgramDiff { site_context: Some(delta), ..Default::default() })
}
