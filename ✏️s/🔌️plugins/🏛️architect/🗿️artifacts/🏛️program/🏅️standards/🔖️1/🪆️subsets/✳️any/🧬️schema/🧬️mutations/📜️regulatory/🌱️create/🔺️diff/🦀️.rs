//! 🔺️ Sparse diff construction for the `create-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::CreateRegulatoryRequirement;
use crate::diff::ProgramRegulatoryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.regulatory_requirement.header.id;
    if base.regulatory.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A regulatory requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.regulatory.len());
    if at > base.regulatory.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the regulatory requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta::insertion(at, payload.regulatory_requirement.clone())), ..Default::default() })
}
