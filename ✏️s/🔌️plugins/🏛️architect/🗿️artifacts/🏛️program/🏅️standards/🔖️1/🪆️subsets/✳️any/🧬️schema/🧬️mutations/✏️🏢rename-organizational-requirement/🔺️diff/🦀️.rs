//! 🔺️ Sparse diff construction for the `rename-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::RenameOrganizationalRequirement;
use crate::artifacts::program::diff::{ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry};
use crate::artifacts::program::registers::OrganizationalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.organizational.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No organizational requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This organizational requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = OrganizationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
