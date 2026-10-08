//! 🔺️ Sparse diff construction for the `replace-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::ReplaceRisk;
use crate::diff::ProgramRisksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.risk.header.id;
    let Some(position) = base.risks.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No risk exists with this id.", [id.0.clone()]);
    };
    if base.risks[position] == payload.risk {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This risk already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramRisksDelta::removal(&base.risks, position);
    delta.absorb(ProgramRisksDelta::insertion(position, payload.risk.clone()));
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(delta), ..Default::default() })
}
