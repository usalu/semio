//! 🔺️ Sparse diff construction for the `create-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::mutation::CreateChangeRecord;
use crate::artifacts::program::diff::ProgramChangesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.changes` on apply.
pub fn diff(payload: &CreateChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { changes: Some(ProgramChangesDelta { added: vec![payload.change_record.clone()], ..Default::default() }), ..Default::default() }
}
