//! 🔺️ Sparse diff construction for the `delete-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::DeletePriorityRecord;
use crate::artifacts::program::diff::ProgramPrioritiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeletePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
