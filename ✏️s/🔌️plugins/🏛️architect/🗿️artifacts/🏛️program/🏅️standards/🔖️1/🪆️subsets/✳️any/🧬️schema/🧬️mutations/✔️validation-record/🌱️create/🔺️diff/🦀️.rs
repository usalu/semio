//! 🔺️ Sparse diff construction for the `create-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::CreateValidationRecord;
use crate::diff::ProgramValidationsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateValidationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.validation_record.header.id;
    if base.validations.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A validation record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.validations.len());
    if at > base.validations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the validation record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { validations: Some(ProgramValidationsDelta::insertion(at, payload.validation_record.clone())), ..Default::default() })
}
