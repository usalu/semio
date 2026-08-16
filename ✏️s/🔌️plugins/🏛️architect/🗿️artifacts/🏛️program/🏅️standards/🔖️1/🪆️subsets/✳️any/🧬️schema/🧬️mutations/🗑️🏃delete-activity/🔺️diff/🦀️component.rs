//! 🔺️ Sparse diff construction for the `delete-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::DeleteActivity;
use crate::artifacts::program::diff::ProgramActivitiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { activities: Some(ProgramActivitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
