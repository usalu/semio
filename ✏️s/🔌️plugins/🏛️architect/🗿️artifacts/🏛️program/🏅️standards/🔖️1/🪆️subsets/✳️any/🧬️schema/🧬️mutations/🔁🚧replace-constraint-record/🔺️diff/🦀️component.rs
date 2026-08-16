//! 🔺️ Sparse diff construction for the `replace-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::ReplaceConstraintRecord;
use crate::artifacts::program::diff::{ProgramConstraintsDelta, ProgramConstraintsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceConstraintRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.constraints.iter().find(|row| row.header.id == payload.constraint_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.constraint_record).expect("diff_patch always produces a full patch");
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.constraint_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
