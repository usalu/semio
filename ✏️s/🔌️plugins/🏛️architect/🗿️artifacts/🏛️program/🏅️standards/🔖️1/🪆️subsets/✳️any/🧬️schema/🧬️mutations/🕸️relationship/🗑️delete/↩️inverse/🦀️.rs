//! ↩️ Inverse (undo) construction for the `delete-relationship` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔗relationships` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteRelationship, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.relationships.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateRelationship(super::super::create_relationship::CreateRelationship { relationship: base.relationships[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
