//! 🔺️ Sparse diff construction for the `create-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::CreateProgramElement;
use crate::artifacts::program::diff::ProgramElementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.elements` on apply.
pub fn diff(payload: &CreateProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { elements: Some(ProgramElementsDelta { added: vec![payload.program_element.clone()], ..Default::default() }), ..Default::default() }
}
