//! ↩️ Inverse (undo) construction for the `create-relationship` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔗relationships` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateRelationship, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRelationship(super::super::delete_relationship::mutation::DeleteRelationship { id: payload.relationship.header.id.clone() })]
}
