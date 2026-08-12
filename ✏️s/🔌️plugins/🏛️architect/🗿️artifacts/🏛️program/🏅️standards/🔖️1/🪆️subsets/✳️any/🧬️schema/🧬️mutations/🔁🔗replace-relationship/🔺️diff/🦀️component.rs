//! 🔺️ Sparse diff construction for the `replace-relationship` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔗relationships` per Wave C.

use super::mutation::ReplaceRelationship;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRelationshipsDelta, ProgramRelationshipsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceRelationship, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.relationships.iter().find(|row| row.header.id == payload.relationship.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.relationship).expect("diff_patch always produces a full patch");
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { patched: vec![ProgramRelationshipsPatchEntry { id: payload.relationship.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
