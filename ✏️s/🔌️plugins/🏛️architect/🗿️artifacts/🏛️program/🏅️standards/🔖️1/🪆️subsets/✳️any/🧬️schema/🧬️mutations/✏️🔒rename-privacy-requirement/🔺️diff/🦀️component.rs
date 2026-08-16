//! 🔺️ Sparse diff construction for the `rename-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::RenamePrivacyRequirement;
use crate::artifacts::program::diff::{ProgramPrivacyDelta, ProgramPrivacyPatchEntry};
use crate::artifacts::program::registers::PrivacyRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenamePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PrivacyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { patched: vec![ProgramPrivacyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
