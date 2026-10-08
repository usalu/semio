//! 🔺️ Sparse diff construction for the `replace-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::ReplaceResource;
use crate::diff::ProgramResourcesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resource.header.id;
    let Some(position) = base.resources.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [id.0.clone()]);
    };
    if base.resources[position] == payload.resource {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This resource already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramResourcesDelta::removal(&base.resources, position);
    delta.absorb(ProgramResourcesDelta::insertion(position, payload.resource.clone()));
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(delta), ..Default::default() })
}
