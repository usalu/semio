//! 🔺️ Sparse diff construction for the `delete-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::DeleteOrganizationalRequirement;
use crate::artifacts::program::diff::ProgramOrganizationalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.organizational.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No organizational requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
