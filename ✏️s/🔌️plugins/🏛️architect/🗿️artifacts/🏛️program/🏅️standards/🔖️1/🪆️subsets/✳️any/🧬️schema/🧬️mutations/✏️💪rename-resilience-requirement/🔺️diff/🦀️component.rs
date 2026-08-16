//! 🔺️ Sparse diff construction for the `rename-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::mutation::RenameResilienceRequirement;
use crate::artifacts::program::diff::{ProgramResilienceDelta, ProgramResiliencePatchEntry};
use crate::artifacts::program::registers::ResilienceRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ResilienceRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { resilience: Some(ProgramResilienceDelta { patched: vec![ProgramResiliencePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
