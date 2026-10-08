//! 🔺️ Sparse diff construction for the `replace-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::ReplaceRequirement;
use crate::diff::ProgramRequirementsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.requirement.header.id;
    let Some(position) = base.requirements.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No requirement exists with this id.", [id.0.clone()]);
    };
    if base.requirements[position] == payload.requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.requirements.len()).then(|| base.requirements.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta { removed: vec![id.0.clone()], added: vec![payload.requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
