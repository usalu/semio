//! 🔺️ Sparse diff construction for the `create-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::CreatePriorityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramPrioritiesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.priorities` on apply.
pub fn diff(payload: &CreatePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { added: vec![payload.priority_record.clone()], ..Default::default() }), ..Default::default() }
}
