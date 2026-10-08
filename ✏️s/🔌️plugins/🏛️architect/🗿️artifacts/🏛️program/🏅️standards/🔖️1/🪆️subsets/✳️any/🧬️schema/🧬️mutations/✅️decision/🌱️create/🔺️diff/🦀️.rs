//! 🔺️ Sparse diff construction for the `create-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::CreateDecision;
use crate::diff::ProgramDecisionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.decision.header.id;
    if base.decisions.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A decision already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.decisions.len());
    if at > base.decisions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the decision list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta::insertion(at, payload.decision.clone())), ..Default::default() })
}
