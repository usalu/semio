//! 🔺️ Sparse diff construction for the `rename-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::mutation::RenameFlowRequirement;
use crate::artifacts::program::diff::{ProgramFlowsDelta, ProgramFlowsPatchEntry};
use crate::artifacts::program::registers::FlowRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameFlowRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = FlowRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { flows: Some(ProgramFlowsDelta { patched: vec![ProgramFlowsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
