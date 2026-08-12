//! 🔺️ Sparse diff construction for the `create-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::CreateActivity;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramActivitiesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.activities` on apply.
pub fn diff(payload: &CreateActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { activities: Some(ProgramActivitiesDelta { added: vec![payload.activity.clone()], ..Default::default() }), ..Default::default() }
}
