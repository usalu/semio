//! 🔺️ Sparse diff construction for the `rename-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::RenameSecurityRequirement;
use crate::artifacts::program::diff::{ProgramSecurityDelta, ProgramSecurityPatchEntry};
use crate::artifacts::program::registers::SecurityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.security.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No security requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This security requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = SecurityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
