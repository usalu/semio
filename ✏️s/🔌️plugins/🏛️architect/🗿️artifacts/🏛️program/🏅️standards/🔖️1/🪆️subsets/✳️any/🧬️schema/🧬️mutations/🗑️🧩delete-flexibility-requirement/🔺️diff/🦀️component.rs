//! 🔺️ Sparse diff construction for the `delete-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::DeleteFlexibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
