//! 🔺️ Sparse diff construction for the `rename-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::RenameValidationRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramValidationsDelta, ProgramValidationsPatchEntry};
use crate::artifacts::program::registers::ValidationRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ValidationRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { validations: Some(ProgramValidationsDelta { patched: vec![ProgramValidationsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
