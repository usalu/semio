//! 🔺️ Sparse diff construction for the `rename-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::RenameProgramElement;
use crate::artifacts::program::diff::{ProgramElementsDelta, ProgramElementsPatchEntry};
use crate::artifacts::program::registers::ProgramElementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ProgramElementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
