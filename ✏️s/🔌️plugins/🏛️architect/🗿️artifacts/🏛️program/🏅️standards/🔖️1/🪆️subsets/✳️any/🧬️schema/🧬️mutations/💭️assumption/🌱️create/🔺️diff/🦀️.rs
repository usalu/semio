//! 🔺️ Sparse diff construction for the `create-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::CreateAssumption;
use crate::standards::v1::subsets::any::schema::diff::ProgramAssumptionsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateAssumption, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.assumption.header.id;
    if base.assumptions.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An assumption already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.assumptions.len());
    if at > base.assumptions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the assumption list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { assumptions: Some(ProgramAssumptionsDelta::insertion(at, payload.assumption.clone())), ..Default::default() })
}
