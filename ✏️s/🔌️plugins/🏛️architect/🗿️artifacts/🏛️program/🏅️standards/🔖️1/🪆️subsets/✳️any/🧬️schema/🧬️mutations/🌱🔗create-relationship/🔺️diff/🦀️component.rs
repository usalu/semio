//! 🔺️ Sparse diff construction for the `create-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::CreateRelationship;
use crate::artifacts::program::diff::ProgramRelationshipsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.relationship.header.id.clone();
    if base.relationships.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A relationship already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(ProgramRelationshipsDelta { added: vec![payload.relationship.clone()], ..Default::default() }), ..Default::default() })
}
