//! 🔺️ Sparse diff construction for the `delete-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::DeleteStakeholder;
use crate::artifacts::program::diff::ProgramStakeholdersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
