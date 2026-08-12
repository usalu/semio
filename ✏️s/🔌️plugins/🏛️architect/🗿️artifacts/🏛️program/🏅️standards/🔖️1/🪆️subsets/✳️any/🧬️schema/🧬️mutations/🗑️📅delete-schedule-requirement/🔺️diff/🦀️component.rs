//! 🔺️ Sparse diff construction for the `delete-schedule-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📅schedules` per Wave C.

use super::mutation::DeleteScheduleRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSchedulesDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteScheduleRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { schedules: Some(ProgramSchedulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
