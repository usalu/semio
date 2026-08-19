//! 🔺️ Sparse diff construction for the `delete-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::mutation::DeleteServiceRequirement;
use crate::artifacts::program::diff::ProgramServicesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteServiceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.services.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No service requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { services: Some(ProgramServicesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
