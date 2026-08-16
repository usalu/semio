//! 🔺️ Sparse diff construction for the `create-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::CreateRegulatoryRequirement;
use crate::artifacts::program::diff::ProgramRegulatoryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.regulatory` on apply.
pub fn diff(payload: &CreateRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { added: vec![payload.regulatory_requirement.clone()], ..Default::default() }), ..Default::default() }
}
