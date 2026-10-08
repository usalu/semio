//! 🔺️ Sparse diff construction for the `create-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::CreateHumanFactorRequirement;
use crate::diff::ProgramHumanFactorsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateHumanFactorRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.human_factor_requirement.header.id;
    if base.human_factors.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A human factor requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.human_factors.len());
    if at > base.human_factors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the human factor requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta::insertion(at, payload.human_factor_requirement.clone())), ..Default::default() })
}
