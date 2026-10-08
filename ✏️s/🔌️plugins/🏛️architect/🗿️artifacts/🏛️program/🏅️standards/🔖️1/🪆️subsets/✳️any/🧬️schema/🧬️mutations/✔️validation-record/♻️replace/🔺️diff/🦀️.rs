//! 🔺️ Sparse diff construction for the `replace-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::ReplaceValidationRecord;
use crate::diff::ProgramValidationsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceValidationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.validation_record.header.id;
    let Some(position) = base.validations.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No validation record exists with this id.", [id.0.clone()]);
    };
    if base.validations[position] == payload.validation_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This validation record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.validations.len()).then(|| base.validations.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { validations: Some(ProgramValidationsDelta { removed: vec![id.0.clone()], added: vec![payload.validation_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
