//! 🔺️ Sparse diff construction for the `replace-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::ReplaceOperationalRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramOperationsDelta, ProgramOperationsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceOperationalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.operations.iter().find(|row| row.header.id == payload.operational_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.operational_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.operational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
