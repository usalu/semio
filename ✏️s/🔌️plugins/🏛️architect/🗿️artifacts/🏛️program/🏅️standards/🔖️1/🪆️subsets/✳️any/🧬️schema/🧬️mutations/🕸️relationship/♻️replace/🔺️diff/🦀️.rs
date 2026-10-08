//! 🔺️ Sparse diff construction for the `replace-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::ReplaceRelationship;
use crate::diff::ProgramRelationshipsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.relationship.header.id;
    let Some(position) = base.relationships.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No relationship exists with this id.", [id.0.clone()]);
    };
    if base.relationships[position] == payload.relationship {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This relationship already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramRelationshipsDelta::removal(&base.relationships, position);
    delta.absorb(ProgramRelationshipsDelta::insertion(position, payload.relationship.clone()));
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(delta), ..Default::default() })
}
