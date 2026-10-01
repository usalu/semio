//! 🔺️ Sparse diff construction for the `create-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::CreateResilienceRequirement;
use crate::diff::ProgramResilienceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateResilienceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.resilience_requirement.header.id.clone();
    if base.resilience.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A resilience requirement already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { resilience: Some(ProgramResilienceDelta { added: vec![payload.resilience_requirement.clone()], ..Default::default() }), ..Default::default() })
}
