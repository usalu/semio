//! 🔺️ Sparse diff construction for the `rename-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::mutation::RenameWayfindingRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramWayfindingDelta, ProgramWayfindingPatchEntry};
use crate::artifacts::program::registers::WayfindingRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameWayfindingRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = WayfindingRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
