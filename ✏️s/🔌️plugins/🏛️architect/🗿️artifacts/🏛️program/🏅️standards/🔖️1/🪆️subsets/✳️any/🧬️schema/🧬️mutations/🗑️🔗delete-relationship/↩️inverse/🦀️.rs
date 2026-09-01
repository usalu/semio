//! ↩️ Inverse (undo) construction for the `delete-relationship` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔗relationships` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteRelationship, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.relationships.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRelationship(super::super::create_relationship::CreateRelationship { relationship: existing.clone() })],
        None => Vec::new(),
    }
}
