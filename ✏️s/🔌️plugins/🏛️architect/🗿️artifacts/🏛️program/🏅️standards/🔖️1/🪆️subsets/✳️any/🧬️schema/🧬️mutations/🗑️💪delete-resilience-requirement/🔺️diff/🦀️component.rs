//! 🔺️ Sparse diff construction for the `delete-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::mutation::DeleteResilienceRequirement;
use crate::artifacts::program::diff::ProgramResilienceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteResilienceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resilience: Some(ProgramResilienceDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
