//! 🔺️ Sparse diff construction for the `replace-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::ReplaceKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramKnowledgeDelta, ProgramKnowledgePatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.knowledge.iter().find(|row| row.header.id == payload.knowledge_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.knowledge_record).expect("diff_patch always produces a full patch");
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { patched: vec![ProgramKnowledgePatchEntry { id: payload.knowledge_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
