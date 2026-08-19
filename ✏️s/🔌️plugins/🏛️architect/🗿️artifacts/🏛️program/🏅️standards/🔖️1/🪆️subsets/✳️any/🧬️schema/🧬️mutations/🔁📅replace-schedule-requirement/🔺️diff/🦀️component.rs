//! 🔺️ Sparse diff construction for the `replace-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::ReplaceScheduleRequirement;
use crate::artifacts::program::diff::{ProgramSchedulesDelta, ProgramSchedulesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceScheduleRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.schedules.iter().find(|row| row.header.id == payload.schedule_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No schedule requirement exists with this id.", [payload.schedule_requirement.header.id.0.clone()]);
    };
    if existing == &payload.schedule_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This schedule requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.schedule_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { schedules: Some(ProgramSchedulesDelta { patched: vec![ProgramSchedulesPatchEntry { id: payload.schedule_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
