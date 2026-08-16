//! 🔺️ Sparse diff construction for the `rename-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::RenameConstraintRecord;
use crate::artifacts::program::diff::{ProgramConstraintsDelta, ProgramConstraintsPatchEntry};
use crate::artifacts::program::registers::ConstraintRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ConstraintRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
