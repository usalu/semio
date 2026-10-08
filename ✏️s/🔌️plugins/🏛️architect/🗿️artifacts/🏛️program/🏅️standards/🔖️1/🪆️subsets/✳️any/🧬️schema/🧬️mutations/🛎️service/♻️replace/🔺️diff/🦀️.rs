//! 🔺️ Sparse diff construction for the `replace-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::ReplaceServiceRequirement;
use crate::diff::ProgramServicesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceServiceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.service_requirement.header.id;
    let Some(position) = base.services.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No service requirement exists with this id.", [id.0.clone()]);
    };
    if base.services[position] == payload.service_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This service requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramServicesDelta::removal(&base.services, position);
    delta.absorb(ProgramServicesDelta::insertion(position, payload.service_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { services: Some(delta), ..Default::default() })
}
