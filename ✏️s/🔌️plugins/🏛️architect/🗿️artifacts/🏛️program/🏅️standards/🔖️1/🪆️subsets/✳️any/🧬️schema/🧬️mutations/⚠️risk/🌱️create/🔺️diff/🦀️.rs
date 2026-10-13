//! 🔺️ Sparse diff construction for the `create-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::CreateRisk;
use crate::standards::v1::subsets::any::schema::diff::ProgramRisksDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.risk.header.id;
    if base.risks.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A risk already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.risks.len());
    if at > base.risks.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the risk list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta::insertion(at, payload.risk.clone())), ..Default::default() })
}
