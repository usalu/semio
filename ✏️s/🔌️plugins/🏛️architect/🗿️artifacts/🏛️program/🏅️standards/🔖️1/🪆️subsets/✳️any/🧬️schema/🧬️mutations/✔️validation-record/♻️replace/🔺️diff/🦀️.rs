//! 🔺️ Sparse diff construction for the `replace-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::ReplaceValidationRecord;
use crate::artifacts::program::diff::{ProgramValidationsDelta, ProgramValidationsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceValidationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.validations.iter().find(|row| row.header.id == payload.validation_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No validation record exists with this id.", [payload.validation_record.header.id.0.clone()]);
    };
    if existing == &payload.validation_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This validation record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.validation_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { validations: Some(ProgramValidationsDelta { patched: vec![ProgramValidationsPatchEntry { id: payload.validation_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
