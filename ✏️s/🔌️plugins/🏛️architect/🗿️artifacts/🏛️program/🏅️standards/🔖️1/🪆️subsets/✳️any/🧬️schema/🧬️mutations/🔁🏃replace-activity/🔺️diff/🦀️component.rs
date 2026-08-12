//! 🔺️ Sparse diff construction for the `replace-activity` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏃activities` per Wave C.

use super::mutation::ReplaceActivity;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramActivitiesDelta, ProgramActivitiesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceActivity, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.activities.iter().find(|row| row.header.id == payload.activity.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.activity).expect("diff_patch always produces a full patch");
    ProgramDiff { activities: Some(ProgramActivitiesDelta { patched: vec![ProgramActivitiesPatchEntry { id: payload.activity.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
