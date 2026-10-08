//! 🔺️ Sparse diff construction for the `replace-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::ReplaceOrganizationalRequirement;
use crate::diff::ProgramOrganizationalDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.organizational_requirement.header.id;
    let Some(position) = base.organizational.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No organizational requirement exists with this id.", [id.0.clone()]);
    };
    if base.organizational[position] == payload.organizational_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This organizational requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.organizational.len()).then(|| base.organizational.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta { removed: vec![id.0.clone()], added: vec![payload.organizational_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
