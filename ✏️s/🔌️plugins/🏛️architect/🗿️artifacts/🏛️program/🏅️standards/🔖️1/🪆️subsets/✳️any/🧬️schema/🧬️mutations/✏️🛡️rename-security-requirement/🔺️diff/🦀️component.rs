//! 🔺️ Sparse diff construction for the `rename-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::RenameSecurityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSecurityDelta, ProgramSecurityPatchEntry};
use crate::artifacts::program::registers::SecurityRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SecurityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
