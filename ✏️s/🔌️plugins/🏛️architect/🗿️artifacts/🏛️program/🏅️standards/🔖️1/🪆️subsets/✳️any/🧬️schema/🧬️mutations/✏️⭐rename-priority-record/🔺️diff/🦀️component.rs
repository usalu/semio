//! 🔺️ Sparse diff construction for the `rename-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::RenamePriorityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramPrioritiesDelta, ProgramPrioritiesPatchEntry};
use crate::artifacts::program::registers::PriorityRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenamePriorityRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PriorityRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { patched: vec![ProgramPrioritiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
