//! 🔺️ Sparse diff construction for the `rename-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::RenameInfrastructureRequirement;
use crate::artifacts::program::diff::{ProgramInfrastructureDelta, ProgramInfrastructurePatchEntry};
use crate::artifacts::program::registers::InfrastructureRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = InfrastructureRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { patched: vec![ProgramInfrastructurePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
