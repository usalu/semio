//! 🔺️ Sparse diff construction for the `rename-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::mutation::RenameStakeholder;
use crate::artifacts::program::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::artifacts::program::registers::StakeholderPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StakeholderPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
