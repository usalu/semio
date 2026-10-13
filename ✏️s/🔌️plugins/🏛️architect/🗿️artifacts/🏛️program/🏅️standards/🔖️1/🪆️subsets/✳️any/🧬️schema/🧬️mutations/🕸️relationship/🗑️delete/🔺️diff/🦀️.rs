//! 🔺️ Sparse diff construction for the `delete-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::DeleteRelationship;
use crate::standards::v1::subsets::any::schema::diff::ProgramRelationshipsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.relationships.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No relationship exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(ProgramRelationshipsDelta::removal(&base.relationships, position)), ..Default::default() })
}
