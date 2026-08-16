//! 🔺️ Sparse diff construction for the `replace-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::ReplaceProgramElement;
use crate::artifacts::program::diff::{ProgramElementsDelta, ProgramElementsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceProgramElement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.elements.iter().find(|row| row.header.id == payload.program_element.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.program_element).expect("diff_patch always produces a full patch");
    ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.program_element.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
