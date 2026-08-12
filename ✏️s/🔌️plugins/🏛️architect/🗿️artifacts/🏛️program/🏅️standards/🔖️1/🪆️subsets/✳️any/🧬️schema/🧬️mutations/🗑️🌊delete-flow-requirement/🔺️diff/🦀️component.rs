//! 🔺️ Sparse diff construction for the `delete-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::mutation::DeleteFlowRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlowsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteFlowRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flows: Some(ProgramFlowsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
