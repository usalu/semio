//! 🔺️ Sparse diff construction for the `delete-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::DeleteCommunicationRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCommunicationDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { communication: Some(ProgramCommunicationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
