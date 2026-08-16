//! 🔺️ Sparse diff construction for the `rename-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::RenameProgramElement;
use crate::artifacts::program::diff::{ProgramElementsDelta, ProgramElementsPatchEntry};
use crate::artifacts::program::registers::ProgramElementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.elements.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This program element already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ProgramElementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
