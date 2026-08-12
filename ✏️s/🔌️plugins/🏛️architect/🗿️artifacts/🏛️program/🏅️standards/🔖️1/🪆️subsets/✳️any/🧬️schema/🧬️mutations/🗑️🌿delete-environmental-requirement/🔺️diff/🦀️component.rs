//! 🔺️ Sparse diff construction for the `delete-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::DeleteEnvironmentalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
