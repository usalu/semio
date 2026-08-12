//! 🔺️ Sparse diff construction for the `rename-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::mutation::RenameServiceRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramServicesDelta, ProgramServicesPatchEntry};
use crate::artifacts::program::registers::ServiceRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ServiceRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { services: Some(ProgramServicesDelta { patched: vec![ProgramServicesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
