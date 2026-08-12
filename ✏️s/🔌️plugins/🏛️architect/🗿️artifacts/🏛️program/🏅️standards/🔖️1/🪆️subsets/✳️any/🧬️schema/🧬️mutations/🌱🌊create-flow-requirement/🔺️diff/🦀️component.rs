//! 🔺️ Sparse diff construction for the `create-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::mutation::CreateFlowRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramFlowsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.flows` on apply.
pub fn diff(payload: &CreateFlowRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flows: Some(ProgramFlowsDelta { added: vec![payload.flow_requirement.clone()], ..Default::default() }), ..Default::default() }
}
