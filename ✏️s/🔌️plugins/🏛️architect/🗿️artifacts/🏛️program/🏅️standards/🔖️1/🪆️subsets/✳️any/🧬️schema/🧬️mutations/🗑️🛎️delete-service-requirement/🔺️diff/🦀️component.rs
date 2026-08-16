//! 🔺️ Sparse diff construction for the `delete-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::mutation::DeleteServiceRequirement;
use crate::artifacts::program::diff::ProgramServicesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { services: Some(ProgramServicesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
