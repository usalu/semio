//! 🔺️ Sparse diff construction for the `rename-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::RenameFlexibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry};
use crate::artifacts::program::registers::FlexibilityRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = FlexibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
