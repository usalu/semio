//! 🔺️ Sparse diff construction for the `rename-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::RenameRegulatoryRequirement;
use crate::artifacts::program::diff::{ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry};
use crate::artifacts::program::registers::RegulatoryRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RegulatoryRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
