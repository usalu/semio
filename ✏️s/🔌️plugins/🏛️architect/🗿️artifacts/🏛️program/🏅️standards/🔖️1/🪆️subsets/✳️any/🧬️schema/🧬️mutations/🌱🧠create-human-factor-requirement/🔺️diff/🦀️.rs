//! 🔺️ Sparse diff construction for the `create-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::CreateHumanFactorRequirement;
use crate::artifacts::program::diff::ProgramHumanFactorsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateHumanFactorRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.human_factor_requirement.header.id.clone();
    if base.human_factors.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A human factor requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { added: vec![payload.human_factor_requirement.clone()], ..Default::default() }), ..Default::default() })
}
