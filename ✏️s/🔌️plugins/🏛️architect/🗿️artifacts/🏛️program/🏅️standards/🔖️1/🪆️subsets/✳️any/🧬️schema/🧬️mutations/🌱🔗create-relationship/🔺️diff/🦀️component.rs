//! 🔺️ Sparse diff construction for the `create-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::CreateRelationship;
use crate::artifacts::program::diff::ProgramRelationshipsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.relationships` on apply.
pub fn diff(payload: &CreateRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { added: vec![payload.relationship.clone()], ..Default::default() }), ..Default::default() }
}
