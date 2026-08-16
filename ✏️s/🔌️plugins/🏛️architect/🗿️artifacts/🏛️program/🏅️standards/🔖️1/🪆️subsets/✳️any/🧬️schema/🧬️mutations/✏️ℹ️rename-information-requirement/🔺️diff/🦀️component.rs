//! 🔺️ Sparse diff construction for the `rename-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::mutation::RenameInformationRequirement;
use crate::artifacts::program::diff::{ProgramInformationDelta, ProgramInformationPatchEntry};
use crate::artifacts::program::registers::InformationRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = InformationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { information: Some(ProgramInformationDelta { patched: vec![ProgramInformationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
