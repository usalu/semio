//! 🔺️ Sparse diff construction for the `rename-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::RenameWorkshop;
use crate::artifacts::program::diff::{ProgramWorkshopsDelta, ProgramWorkshopsPatchEntry};
use crate::artifacts::program::registers::WorkshopPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = WorkshopPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { patched: vec![ProgramWorkshopsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
