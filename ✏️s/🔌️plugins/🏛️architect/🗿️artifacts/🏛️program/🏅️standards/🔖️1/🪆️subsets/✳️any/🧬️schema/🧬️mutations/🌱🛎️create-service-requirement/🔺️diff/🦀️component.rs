//! 🔺️ Sparse diff construction for the `create-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::mutation::CreateServiceRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramServicesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.services` on apply.
pub fn diff(payload: &CreateServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { services: Some(ProgramServicesDelta { added: vec![payload.service_requirement.clone()], ..Default::default() }), ..Default::default() }
}
