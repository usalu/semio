//! 🔺️ Sparse diff construction for the `replace-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::ReplaceResource;
use crate::diff::ProgramResourcesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resource.header.id;
    let Some(position) = base.resources.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [id.0.clone()]);
    };
    if base.resources[position] == payload.resource {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This resource already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.resources.len()).then(|| base.resources.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { removed: vec![id.0.clone()], added: vec![payload.resource.clone()], reordered, ..Default::default() }), ..Default::default() })
}
