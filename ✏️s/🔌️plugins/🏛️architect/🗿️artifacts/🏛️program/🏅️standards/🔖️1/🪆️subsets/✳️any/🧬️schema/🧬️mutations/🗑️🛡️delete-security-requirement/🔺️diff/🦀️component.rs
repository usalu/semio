//! 🔺️ Sparse diff construction for the `delete-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::DeleteSecurityRequirement;
use crate::artifacts::program::diff::ProgramSecurityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.security.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No security requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { security: Some(ProgramSecurityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
