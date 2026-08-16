//! 🔺️ Sparse diff construction for the `delete-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::DeleteValidationRecord;
use crate::artifacts::program::diff::ProgramValidationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteValidationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.validations.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No validation record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { validations: Some(ProgramValidationsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
