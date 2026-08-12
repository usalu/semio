//! 🔺️ Sparse diff construction for the `create-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::CreateCommunicationRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCommunicationDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.communication` on apply.
pub fn diff(payload: &CreateCommunicationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { communication: Some(ProgramCommunicationDelta { added: vec![payload.communication_requirement.clone()], ..Default::default() }), ..Default::default() }
}
