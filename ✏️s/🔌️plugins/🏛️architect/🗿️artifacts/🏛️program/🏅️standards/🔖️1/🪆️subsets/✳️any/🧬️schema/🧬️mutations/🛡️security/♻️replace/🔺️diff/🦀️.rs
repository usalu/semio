//! 🔺️ Sparse diff construction for the `replace-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::ReplaceSecurityRequirement;
use crate::diff::ProgramSecurityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceSecurityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.security_requirement.header.id;
    let Some(position) = base.security.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No security requirement exists with this id.", [id.0.clone()]);
    };
    if base.security[position] == payload.security_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This security requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramSecurityDelta::removal(&base.security, position);
    delta.absorb(ProgramSecurityDelta::insertion(position, payload.security_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { security: Some(delta), ..Default::default() })
}
