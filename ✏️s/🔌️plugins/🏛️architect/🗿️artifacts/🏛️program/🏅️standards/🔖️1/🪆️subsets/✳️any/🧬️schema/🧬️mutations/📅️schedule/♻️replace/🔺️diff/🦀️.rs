//! 🔺️ Sparse diff construction for the `replace-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::ReplaceScheduleRequirement;
use crate::diff::ProgramSchedulesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.schedule_requirement.header.id;
    let Some(position) = base.schedules.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No schedule requirement exists with this id.", [id.0.clone()]);
    };
    if base.schedules[position] == payload.schedule_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This schedule requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.schedules.len()).then(|| base.schedules.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta { removed: vec![id.0.clone()], added: vec![payload.schedule_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
