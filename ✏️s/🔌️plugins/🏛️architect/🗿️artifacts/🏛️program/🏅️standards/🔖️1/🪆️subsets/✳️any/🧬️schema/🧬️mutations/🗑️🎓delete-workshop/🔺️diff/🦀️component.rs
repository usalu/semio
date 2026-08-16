//! 🔺️ Sparse diff construction for the `delete-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::DeleteWorkshop;
use crate::artifacts::program::diff::ProgramWorkshopsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
