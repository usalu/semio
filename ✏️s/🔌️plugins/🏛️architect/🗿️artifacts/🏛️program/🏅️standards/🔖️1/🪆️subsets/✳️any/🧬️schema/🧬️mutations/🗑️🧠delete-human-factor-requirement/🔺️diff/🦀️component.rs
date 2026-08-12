//! 🔺️ Sparse diff construction for the `delete-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::mutation::DeleteHumanFactorRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
