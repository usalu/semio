//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::DeleteKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramKnowledgeDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteKnowledgeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
