//! 🔺️ Sparse diff construction for the `replace-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::ReplaceStakeholder;
use crate::artifacts::program::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceStakeholder, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.stakeholders.iter().find(|row| row.header.id == payload.stakeholder.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.stakeholder).expect("diff_patch always produces a full patch");
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.stakeholder.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
