//! 🔺️ Sparse diff construction for the `relationships` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateRelationship, DeleteRelationship, RenameRelationship, ReplaceRelationship};
use crate::artifacts::program::diff::{ProgramRelationshipsDelta, ProgramRelationshipsPatchEntry};
use crate::artifacts::program::registers::RelationshipPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.relationships` on apply.
pub fn diff_create(payload: &CreateRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { added: vec![payload.relationship.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameRelationship, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RelationshipPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { patched: vec![ProgramRelationshipsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceRelationship, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.relationships.iter().find(|row| row.header.id == payload.relationship.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.relationship).expect("diff_patch always produces a full patch");
    ProgramDiff { relationships: Some(ProgramRelationshipsDelta { patched: vec![ProgramRelationshipsPatchEntry { id: payload.relationship.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
