//! 🔺️ Sparse diff construction for the `replace-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::ReplaceRegulatoryRequirement;
use crate::diff::ProgramRegulatoryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.regulatory_requirement.header.id;
    let Some(position) = base.regulatory.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No regulatory requirement exists with this id.", [id.0.clone()]);
    };
    if base.regulatory[position] == payload.regulatory_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This regulatory requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.regulatory.len()).then(|| base.regulatory.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { removed: vec![id.0.clone()], added: vec![payload.regulatory_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
