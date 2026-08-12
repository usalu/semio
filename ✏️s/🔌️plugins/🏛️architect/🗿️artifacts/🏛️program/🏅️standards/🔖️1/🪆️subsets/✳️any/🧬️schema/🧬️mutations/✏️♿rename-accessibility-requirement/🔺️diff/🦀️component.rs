//! 🔺️ Sparse diff construction for the `rename-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::mutation::RenameAccessibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAccessibilityDelta, ProgramAccessibilityPatchEntry};
use crate::artifacts::program::registers::AccessibilityRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AccessibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { patched: vec![ProgramAccessibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
