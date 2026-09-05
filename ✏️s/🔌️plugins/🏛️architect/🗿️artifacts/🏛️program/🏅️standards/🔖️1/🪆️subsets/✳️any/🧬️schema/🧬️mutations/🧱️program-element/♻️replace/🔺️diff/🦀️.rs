//! 🔺️ Sparse diff construction for the `replace-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::ReplaceProgramElement;
use crate::artifacts::program::diff::{ProgramElementsDelta, ProgramElementsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.elements.iter().find(|row| row.header.id == payload.program_element.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [payload.program_element.header.id.0.clone()]);
    };
    if existing == &payload.program_element {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This program element already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.program_element).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.program_element.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
