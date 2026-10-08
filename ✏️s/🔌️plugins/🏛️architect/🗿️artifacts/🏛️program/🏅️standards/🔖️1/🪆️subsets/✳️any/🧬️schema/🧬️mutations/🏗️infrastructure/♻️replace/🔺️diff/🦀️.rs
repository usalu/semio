//! 🔺️ Sparse diff construction for the `replace-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::ReplaceInfrastructureRequirement;
use crate::diff::ProgramInfrastructureDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.infrastructure_requirement.header.id;
    let Some(position) = base.infrastructure.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No infrastructure requirement exists with this id.", [id.0.clone()]);
    };
    if base.infrastructure[position] == payload.infrastructure_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This infrastructure requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramInfrastructureDelta::removal(&base.infrastructure, position);
    delta.absorb(ProgramInfrastructureDelta::insertion(position, payload.infrastructure_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { infrastructure: Some(delta), ..Default::default() })
}
