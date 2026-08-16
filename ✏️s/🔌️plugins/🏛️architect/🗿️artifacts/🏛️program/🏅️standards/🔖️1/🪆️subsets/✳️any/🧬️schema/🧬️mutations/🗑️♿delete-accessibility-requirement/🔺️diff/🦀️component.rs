//! 🔺️ Sparse diff construction for the `delete-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::mutation::DeleteAccessibilityRequirement;
use crate::artifacts::program::diff::ProgramAccessibilityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
