//! 🔺️ Sparse diff construction for the `knowledge` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateKnowledgeRecord, DeleteKnowledgeRecord, RenameKnowledgeRecord, ReplaceKnowledgeRecord};
use crate::artifacts::program::diff::{ProgramKnowledgeDelta, ProgramKnowledgePatchEntry};
use crate::artifacts::program::registers::KnowledgeRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.knowledge` on apply.
pub fn diff_create(payload: &CreateKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { added: vec![payload.knowledge_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = KnowledgeRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { patched: vec![ProgramKnowledgePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.knowledge.iter().find(|row| row.header.id == payload.knowledge_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.knowledge_record).expect("diff_patch always produces a full patch");
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { patched: vec![ProgramKnowledgePatchEntry { id: payload.knowledge_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
