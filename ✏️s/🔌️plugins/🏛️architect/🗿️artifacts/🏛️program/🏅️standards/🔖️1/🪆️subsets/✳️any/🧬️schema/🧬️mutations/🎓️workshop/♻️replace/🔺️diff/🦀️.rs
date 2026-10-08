//! 🔺️ Sparse diff construction for the `replace-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::ReplaceWorkshop;
use crate::diff::ProgramWorkshopsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.workshop.header.id;
    let Some(position) = base.workshops.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No workshop exists with this id.", [id.0.clone()]);
    };
    if base.workshops[position] == payload.workshop {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This workshop already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.workshops.len()).then(|| base.workshops.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(ProgramWorkshopsDelta { removed: vec![id.0.clone()], added: vec![payload.workshop.clone()], reordered, ..Default::default() }), ..Default::default() })
}
