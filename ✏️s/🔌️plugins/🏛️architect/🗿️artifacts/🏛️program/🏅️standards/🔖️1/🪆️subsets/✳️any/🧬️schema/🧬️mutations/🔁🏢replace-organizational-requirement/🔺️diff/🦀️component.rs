//! 🔺️ Sparse diff construction for the `replace-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::ReplaceOrganizationalRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.organizational_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.organizational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
