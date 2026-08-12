//! 🔺️ Sparse diff construction for the `delete-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::DeleteRegulatoryRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRegulatoryDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
