//! 🔺️ Sparse diff construction for the `create-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::mutation::CreateResilienceRequirement;
use crate::artifacts::program::diff::ProgramResilienceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.resilience` on apply.
pub fn diff(payload: &CreateResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resilience: Some(ProgramResilienceDelta { added: vec![payload.resilience_requirement.clone()], ..Default::default() }), ..Default::default() }
}
