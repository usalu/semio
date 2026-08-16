//! 🔺️ Sparse diff construction for the `rename-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::RenameRelationship;
use crate::artifacts::program::diff::{ProgramRelationshipsDelta, ProgramRelationshipsPatchEntry};
use crate::artifacts::program::registers::RelationshipPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RelationshipPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { patched: vec![ProgramRelationshipsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
