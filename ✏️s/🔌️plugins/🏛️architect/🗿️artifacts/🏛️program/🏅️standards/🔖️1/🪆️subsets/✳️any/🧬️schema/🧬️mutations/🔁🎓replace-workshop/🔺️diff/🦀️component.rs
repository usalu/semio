//! 🔺️ Sparse diff construction for the `replace-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::ReplaceWorkshop;
use crate::artifacts::program::diff::{ProgramWorkshopsDelta, ProgramWorkshopsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.workshops.iter().find(|row| row.header.id == payload.workshop.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No workshop exists with this id.", [payload.workshop.header.id.0.clone()]);
    };
    if existing == &payload.workshop {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This workshop already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.workshop).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(ProgramWorkshopsDelta { patched: vec![ProgramWorkshopsPatchEntry { id: payload.workshop.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
