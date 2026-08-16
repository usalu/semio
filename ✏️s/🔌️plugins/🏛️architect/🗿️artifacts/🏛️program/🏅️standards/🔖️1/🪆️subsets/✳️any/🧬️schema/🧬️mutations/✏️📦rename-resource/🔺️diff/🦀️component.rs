//! 🔺️ Sparse diff construction for the `rename-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::RenameResource;
use crate::artifacts::program::diff::{ProgramResourcesDelta, ProgramResourcesPatchEntry};
use crate::artifacts::program::registers::ResourcePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.resources.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This resource already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ResourcePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
