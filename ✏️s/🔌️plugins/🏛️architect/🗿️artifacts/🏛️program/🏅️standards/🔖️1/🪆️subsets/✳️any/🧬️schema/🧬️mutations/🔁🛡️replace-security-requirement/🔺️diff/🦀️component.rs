//! 🔺️ Sparse diff construction for the `replace-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::ReplaceSecurityRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSecurityDelta, ProgramSecurityPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSecurityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.security.iter().find(|row| row.header.id == payload.security_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.security_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.security_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
