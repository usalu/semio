//! 🔺️ Sparse diff construction for the `create-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::CreateInfrastructureRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramInfrastructureDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.infrastructure` on apply.
pub fn diff(payload: &CreateInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { added: vec![payload.infrastructure_requirement.clone()], ..Default::default() }), ..Default::default() }
}
