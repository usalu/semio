//! 🔺️ Sparse diff construction for the `create-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::CreateWorkshop;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramWorkshopsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.workshops` on apply.
pub fn diff(payload: &CreateWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { added: vec![payload.workshop.clone()], ..Default::default() }), ..Default::default() }
}
