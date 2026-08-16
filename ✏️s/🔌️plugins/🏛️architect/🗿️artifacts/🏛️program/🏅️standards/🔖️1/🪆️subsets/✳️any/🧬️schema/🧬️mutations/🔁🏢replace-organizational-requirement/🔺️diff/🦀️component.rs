//! 🔺️ Sparse diff construction for the `replace-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::ReplaceOrganizationalRequirement;
use crate::artifacts::program::diff::{ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No organizational requirement exists with this id.", [payload.organizational_requirement.header.id.0.clone()]);
    };
    if existing == &payload.organizational_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This organizational requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.organizational_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.organizational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
