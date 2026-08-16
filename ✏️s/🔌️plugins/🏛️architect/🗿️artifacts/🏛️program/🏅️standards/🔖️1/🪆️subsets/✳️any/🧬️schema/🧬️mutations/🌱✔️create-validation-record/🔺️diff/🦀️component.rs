//! 🔺️ Sparse diff construction for the `create-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::CreateValidationRecord;
use crate::artifacts::program::diff::ProgramValidationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.validations` on apply.
pub fn diff(payload: &CreateValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { validations: Some(ProgramValidationsDelta { added: vec![payload.validation_record.clone()], ..Default::default() }), ..Default::default() }
}
