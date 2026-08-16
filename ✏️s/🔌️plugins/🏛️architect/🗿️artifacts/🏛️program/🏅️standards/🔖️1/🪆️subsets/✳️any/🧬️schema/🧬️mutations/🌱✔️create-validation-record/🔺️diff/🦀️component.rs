//! 🔺️ Sparse diff construction for the `create-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::CreateValidationRecord;
use crate::artifacts::program::diff::ProgramValidationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateValidationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.validation_record.header.id.clone();
    if base.validations.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A validation record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { validations: Some(ProgramValidationsDelta { added: vec![payload.validation_record.clone()], ..Default::default() }), ..Default::default() })
}
