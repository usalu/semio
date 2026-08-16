//! 🔺️ Sparse diff construction for the `create-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::CreateStakeholder;
use crate::artifacts::program::diff::ProgramStakeholdersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.stakeholders` on apply.
pub fn diff(payload: &CreateStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { added: vec![payload.stakeholder.clone()], ..Default::default() }), ..Default::default() }
}
