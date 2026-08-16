//! 🔺️ Sparse diff construction for the `delete-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::DeleteInfrastructureRequirement;
use crate::artifacts::program::diff::ProgramInfrastructureDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteInfrastructureRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
