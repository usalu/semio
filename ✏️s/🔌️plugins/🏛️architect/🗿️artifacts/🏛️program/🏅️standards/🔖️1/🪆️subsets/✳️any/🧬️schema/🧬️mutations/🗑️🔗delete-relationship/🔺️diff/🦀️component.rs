//! 🔺️ Sparse diff construction for the `delete-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::DeleteRelationship;
use crate::artifacts::program::diff::ProgramRelationshipsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
