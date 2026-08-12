//! 🔺️ Sparse diff construction for the `rename-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::mutation::RenameChangeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramChangesDelta, ProgramChangesPatchEntry};
use crate::artifacts::program::registers::ChangeRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ChangeRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { changes: Some(ProgramChangesDelta { patched: vec![ProgramChangesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
