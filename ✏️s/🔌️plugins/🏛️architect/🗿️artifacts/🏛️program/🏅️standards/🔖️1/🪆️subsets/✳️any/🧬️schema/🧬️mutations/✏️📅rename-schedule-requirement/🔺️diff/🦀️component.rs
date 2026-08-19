//! 🔺️ Sparse diff construction for the `rename-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::RenameScheduleRequirement;
use crate::artifacts::program::diff::{ProgramSchedulesDelta, ProgramSchedulesPatchEntry};
use crate::artifacts::program::registers::ScheduleRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.schedules.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No schedule requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This schedule requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ScheduleRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta { patched: vec![ProgramSchedulesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
