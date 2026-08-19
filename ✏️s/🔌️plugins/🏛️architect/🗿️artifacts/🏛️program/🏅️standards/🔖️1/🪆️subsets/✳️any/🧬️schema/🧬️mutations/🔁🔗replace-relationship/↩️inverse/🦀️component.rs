//! ↩️ Inverse (undo) construction for the `replace-relationship` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔗relationships` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceRelationship, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.relationships.iter().find(|row| row.header.id == payload.relationship.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRelationship(super::mutation::ReplaceRelationship { relationship: existing.clone() })],
        None => Vec::new(),
    }
}
