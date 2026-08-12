//! 🔺️ Sparse diff construction for the `create-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::mutation::CreateInformationRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramInformationDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.information` on apply.
pub fn diff(payload: &CreateInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { information: Some(ProgramInformationDelta { added: vec![payload.information_requirement.clone()], ..Default::default() }), ..Default::default() }
}
