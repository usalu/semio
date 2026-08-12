//! 🔺️ Sparse diff construction for the `rename-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::RenameCommunicationRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCommunicationDelta, ProgramCommunicationPatchEntry};
use crate::artifacts::program::registers::CommunicationRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CommunicationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
