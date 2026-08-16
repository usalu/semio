//! 🔺️ Sparse diff construction for the `replace-validation-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✔️validations` per Wave C.

use super::mutation::ReplaceValidationRecord;
use crate::artifacts::program::diff::{ProgramValidationsDelta, ProgramValidationsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceValidationRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.validations.iter().find(|row| row.header.id == payload.validation_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.validation_record).expect("diff_patch always produces a full patch");
    ProgramDiff { validations: Some(ProgramValidationsDelta { patched: vec![ProgramValidationsPatchEntry { id: payload.validation_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
