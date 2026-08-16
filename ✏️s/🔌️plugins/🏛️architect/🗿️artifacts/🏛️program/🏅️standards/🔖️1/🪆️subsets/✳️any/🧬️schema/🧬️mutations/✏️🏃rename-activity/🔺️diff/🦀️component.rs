//! 🔺️ Sparse diff construction for the `rename-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::RenameActivity;
use crate::artifacts::program::diff::{ProgramActivitiesDelta, ProgramActivitiesPatchEntry};
use crate::artifacts::program::registers::ActivityPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameActivity, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ActivityPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
