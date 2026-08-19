//! 🔺️ Sparse diff construction for the `rename-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::RenameRisk;
use crate::artifacts::program::diff::{ProgramRisksDelta, ProgramRisksPatchEntry};
use crate::artifacts::program::registers::RiskPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.risks.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No risk exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This risk already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = RiskPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
