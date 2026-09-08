//! 🔺️ Sparse diff construction for the `create-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::CreateRisk;
use crate::diff::ProgramRisksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.risk.header.id.clone();
    if base.risks.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A risk already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta { added: vec![payload.risk.clone()], ..Default::default() }), ..Default::default() })
}
