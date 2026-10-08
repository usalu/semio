//! 🔺️ Sparse diff construction for the `create-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::CreateRelationship;
use crate::diff::ProgramRelationshipsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.relationship.header.id;
    if base.relationships.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A relationship already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.relationships.len());
    if at > base.relationships.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the relationship list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(ProgramRelationshipsDelta::insertion(at, payload.relationship.clone())), ..Default::default() })
}
