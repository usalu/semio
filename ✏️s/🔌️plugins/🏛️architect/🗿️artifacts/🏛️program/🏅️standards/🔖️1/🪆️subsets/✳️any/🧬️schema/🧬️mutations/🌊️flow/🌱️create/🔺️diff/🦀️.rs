//! 🔺️ Sparse diff construction for the `create-flow-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌊flows` per Wave C.

use super::CreateFlowRequirement;
use crate::diff::ProgramFlowsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateFlowRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flow_requirement.header.id;
    if base.flows.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A flow requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.flows.len());
    if at > base.flows.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the flow requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { flows: Some(ProgramFlowsDelta::insertion(at, payload.flow_requirement.clone())), ..Default::default() })
}
