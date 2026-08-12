//! 🔺️ Sparse diff construction for the `create-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::mutation::CreateAccessibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAccessibilityDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.accessibility` on apply.
pub fn diff(payload: &CreateAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { added: vec![payload.accessibility_requirement.clone()], ..Default::default() }), ..Default::default() }
}
