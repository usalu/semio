//! 🔺️ Sparse diff construction for the `rename-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::RenameRelationship;
use crate::artifacts::program::diff::{ProgramRelationshipsDelta, ProgramRelationshipsPatchEntry};
use crate::artifacts::program::registers::RelationshipPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameRelationship, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.relationships.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No relationship exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This relationship already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = RelationshipPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { relationships: Some(ProgramRelationshipsDelta { patched: vec![ProgramRelationshipsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
