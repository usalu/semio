//! 🔺️ Sparse diff construction for the `rename-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::RenameWorkshop;
use crate::artifacts::program::diff::{ProgramWorkshopsDelta, ProgramWorkshopsPatchEntry};
use crate::artifacts::program::registers::WorkshopPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.workshops.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No workshop exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This workshop already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = WorkshopPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(ProgramWorkshopsDelta { patched: vec![ProgramWorkshopsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
