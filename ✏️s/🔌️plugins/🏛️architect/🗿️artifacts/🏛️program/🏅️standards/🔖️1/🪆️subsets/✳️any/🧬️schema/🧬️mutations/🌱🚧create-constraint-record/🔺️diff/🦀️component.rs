//! 🔺️ Sparse diff construction for the `create-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::CreateConstraintRecord;
use crate::artifacts::program::diff::ProgramConstraintsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.constraints` on apply.
pub fn diff(payload: &CreateConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { added: vec![payload.constraint_record.clone()], ..Default::default() }), ..Default::default() }
}
