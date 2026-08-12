//! 🔺️ Sparse diff construction for the `create-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::CreateScheduleRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSchedulesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.schedules` on apply.
pub fn diff(payload: &CreateScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { added: vec![payload.schedule_requirement.clone()], ..Default::default() }), ..Default::default() }
}
