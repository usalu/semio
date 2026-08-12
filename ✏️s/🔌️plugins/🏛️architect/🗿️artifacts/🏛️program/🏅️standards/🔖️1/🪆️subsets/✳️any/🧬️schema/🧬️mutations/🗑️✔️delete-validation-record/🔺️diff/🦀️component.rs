//! 🔺️ Sparse diff construction for the `delete-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::DeleteValidationRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramValidationsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteValidationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { validations: Some(ProgramValidationsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
