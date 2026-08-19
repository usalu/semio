//! 🔺️ Sparse diff construction for the `delete-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::mutation::DeleteFlexibilityRequirement;
use crate::artifacts::program::diff::ProgramFlexibilityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.flexibility.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flexibility requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
