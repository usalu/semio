//! 🔺️ Sparse diff construction for the `create-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::CreateFlexibilityRequirement;
use crate::artifacts::program::diff::ProgramFlexibilityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.flexibility_requirement.header.id.clone();
    if base.flexibility.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A flexibility requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { added: vec![payload.flexibility_requirement.clone()], ..Default::default() }), ..Default::default() })
}
