//! 🔺️ Sparse diff construction for the `create-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::mutation::CreateWayfindingRequirement;
use crate::artifacts::program::diff::ProgramWayfindingDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.wayfinding` on apply.
pub fn diff(payload: &CreateWayfindingRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { added: vec![payload.wayfinding_requirement.clone()], ..Default::default() }), ..Default::default() }
}
