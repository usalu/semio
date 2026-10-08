//! 🔺️ Sparse diff construction for the `replace-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::ReplaceInfrastructureRequirement;
use crate::diff::ProgramInfrastructureDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.infrastructure_requirement.header.id;
    let Some(position) = base.infrastructure.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No infrastructure requirement exists with this id.", [id.0.clone()]);
    };
    if base.infrastructure[position] == payload.infrastructure_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This infrastructure requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.infrastructure.len()).then(|| base.infrastructure.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { removed: vec![id.0.clone()], added: vec![payload.infrastructure_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
