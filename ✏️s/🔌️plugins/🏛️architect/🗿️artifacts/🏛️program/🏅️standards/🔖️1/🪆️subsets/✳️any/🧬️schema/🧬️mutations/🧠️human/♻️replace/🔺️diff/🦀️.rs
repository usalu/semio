//! 🔺️ Sparse diff construction for the `replace-human-factor-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧠human-factors` per Wave C.

use super::ReplaceHumanFactorRequirement;
use crate::diff::ProgramHumanFactorsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.human_factor_requirement.header.id;
    let Some(position) = base.human_factors.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No human factor requirement exists with this id.", [id.0.clone()]);
    };
    if base.human_factors[position] == payload.human_factor_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This human factor requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.human_factors.len()).then(|| base.human_factors.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { removed: vec![id.0.clone()], added: vec![payload.human_factor_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
