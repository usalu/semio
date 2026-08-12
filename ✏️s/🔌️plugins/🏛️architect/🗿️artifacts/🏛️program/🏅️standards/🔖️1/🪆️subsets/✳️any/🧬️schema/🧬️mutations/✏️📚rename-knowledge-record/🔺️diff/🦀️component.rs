//! 🔺️ Sparse diff construction for the `rename-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::RenameKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramKnowledgeDelta, ProgramKnowledgePatchEntry};
use crate::artifacts::program::registers::KnowledgeRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = KnowledgeRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { patched: vec![ProgramKnowledgePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
