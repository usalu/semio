//! 🔺️ Sparse diff construction for the `create-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::mutation::CreateHumanFactorRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.human_factors` on apply.
pub fn diff(payload: &CreateHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { added: vec![payload.human_factor_requirement.clone()], ..Default::default() }), ..Default::default() }
}
