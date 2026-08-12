//! 🔺️ Sparse diff construction for the `create-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::CreateKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramKnowledgeDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.knowledge` on apply.
pub fn diff(payload: &CreateKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { added: vec![payload.knowledge_record.clone()], ..Default::default() }), ..Default::default() }
}
