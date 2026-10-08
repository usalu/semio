//! 🔺️ Sparse diff construction for the `create-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::CreateRisk;
use crate::diff::ProgramRisksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.risk.header.id;
    if base.risks.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A risk already exists with this id.", [id.0.clone()]);
    }
    let length = base.risks.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the risk list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.risks.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta { added: vec![payload.risk.clone()], reordered, ..Default::default() }), ..Default::default() })
}
