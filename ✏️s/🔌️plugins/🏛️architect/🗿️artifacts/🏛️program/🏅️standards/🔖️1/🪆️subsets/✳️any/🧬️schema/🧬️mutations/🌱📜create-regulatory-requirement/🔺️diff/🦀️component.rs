//! 🔺️ Sparse diff construction for the `create-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::CreateRegulatoryRequirement;
use crate::artifacts::program::diff::ProgramRegulatoryDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.regulatory_requirement.header.id.clone();
    if base.regulatory.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A regulatory requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { added: vec![payload.regulatory_requirement.clone()], ..Default::default() }), ..Default::default() })
}
