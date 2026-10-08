//! 🔺️ Sparse diff construction for the `replace-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::ReplaceWorkshop;
use crate::diff::ProgramWorkshopsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.workshop.header.id;
    let Some(position) = base.workshops.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No workshop exists with this id.", [id.0.clone()]);
    };
    if base.workshops[position] == payload.workshop {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This workshop already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramWorkshopsDelta::removal(&base.workshops, position);
    delta.absorb(ProgramWorkshopsDelta::insertion(position, payload.workshop.clone()));
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(delta), ..Default::default() })
}
