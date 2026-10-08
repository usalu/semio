//! 🔺️ Sparse diff construction for the `replace-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::ReplaceProgramElement;
use crate::diff::ProgramElementsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.program_element.header.id;
    let Some(position) = base.elements.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No program element exists with this id.", [id.0.clone()]);
    };
    if base.elements[position] == payload.program_element {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This program element already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramElementsDelta::removal(&base.elements, position);
    delta.absorb(ProgramElementsDelta::insertion(position, payload.program_element.clone()));
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(delta), ..Default::default() })
}
