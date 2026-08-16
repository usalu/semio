//! 🔺️ Sparse diff construction for the `delete-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::mutation::DeleteInformationRequirement;
use crate::artifacts::program::diff::ProgramInformationDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { information: Some(ProgramInformationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
