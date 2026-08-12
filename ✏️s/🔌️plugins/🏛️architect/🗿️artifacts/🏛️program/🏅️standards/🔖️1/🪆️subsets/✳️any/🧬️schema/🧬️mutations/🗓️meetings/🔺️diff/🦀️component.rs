//! 🔺️ Sparse diff construction for the `meetings` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateMeetingRecord, DeleteMeetingRecord, RenameMeetingRecord, ReplaceMeetingRecord};
use crate::artifacts::program::diff::{ProgramMeetingsDelta, ProgramMeetingsPatchEntry};
use crate::artifacts::program::registers::MeetingRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.meetings` on apply.
pub fn diff_create(payload: &CreateMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { added: vec![payload.meeting_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = MeetingRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { patched: vec![ProgramMeetingsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceMeetingRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.meetings.iter().find(|row| row.header.id == payload.meeting_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.meeting_record).expect("diff_patch always produces a full patch");
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { patched: vec![ProgramMeetingsPatchEntry { id: payload.meeting_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
