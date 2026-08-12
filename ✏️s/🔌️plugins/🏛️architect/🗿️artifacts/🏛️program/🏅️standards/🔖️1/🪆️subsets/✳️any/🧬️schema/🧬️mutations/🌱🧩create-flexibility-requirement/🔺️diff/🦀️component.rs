//! 🔺️ Sparse diff construction for the `create-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::CreateFlexibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.flexibility` on apply.
pub fn diff(payload: &CreateFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { added: vec![payload.flexibility_requirement.clone()], ..Default::default() }), ..Default::default() }
}
