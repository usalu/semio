//! 🔺️ Sparse diff construction for the `create-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::CreateHumanFactorRequirement;
use crate::diff::ProgramHumanFactorsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateHumanFactorRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.human_factor_requirement.header.id;
    if base.human_factors.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A human factor requirement already exists with this id.", [id.0.clone()]);
    }
    let length = base.human_factors.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the human factor requirement list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.human_factors.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { added: vec![payload.human_factor_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
