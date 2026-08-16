//! 🔺️ Sparse diff construction for the `rename-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::RenameScheduleRequirement;
use crate::artifacts::program::diff::{ProgramSchedulesDelta, ProgramSchedulesPatchEntry};
use crate::artifacts::program::registers::ScheduleRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ScheduleRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { patched: vec![ProgramSchedulesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
