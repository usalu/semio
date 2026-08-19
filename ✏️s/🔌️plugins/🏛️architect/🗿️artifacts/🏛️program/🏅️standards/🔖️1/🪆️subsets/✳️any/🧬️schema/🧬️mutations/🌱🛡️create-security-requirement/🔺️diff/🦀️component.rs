//! 🔺️ Sparse diff construction for the `create-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::CreateSecurityRequirement;
use crate::artifacts::program::diff::ProgramSecurityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.security_requirement.header.id.clone();
    if base.security.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A security requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { security: Some(ProgramSecurityDelta { added: vec![payload.security_requirement.clone()], ..Default::default() }), ..Default::default() })
}
