//! 🔺️ Sparse diff construction for the `replace-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::ReplaceSecurityRequirement;
use crate::artifacts::program::diff::{ProgramSecurityDelta, ProgramSecurityPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.security.iter().find(|row| row.header.id == payload.security_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No security requirement exists with this id.", [payload.security_requirement.header.id.0.clone()]);
    };
    if existing == &payload.security_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This security requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.security_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.security_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
