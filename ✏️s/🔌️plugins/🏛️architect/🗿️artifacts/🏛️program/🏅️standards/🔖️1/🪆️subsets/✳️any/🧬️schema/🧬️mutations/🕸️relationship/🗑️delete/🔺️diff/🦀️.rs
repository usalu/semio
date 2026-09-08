//! 🔺️ Sparse diff construction for the `delete-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::DeleteRelationship;
use crate::diff::ProgramRelationshipsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.relationships.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No relationship exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(ProgramRelationshipsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
