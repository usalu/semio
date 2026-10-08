//! 🔺️ Sparse diff construction for the `create-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::CreateSecurityRequirement;
use crate::diff::ProgramSecurityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.security_requirement.header.id;
    if base.security.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A security requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.security.len());
    if at > base.security.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the security requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { security: Some(ProgramSecurityDelta::insertion(at, payload.security_requirement.clone())), ..Default::default() })
}
