//! 🔺️ Sparse diff construction for the `replace-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::ReplaceRisk;
use crate::artifacts::program::diff::{ProgramRisksDelta, ProgramRisksPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.risks.iter().find(|row| row.header.id == payload.risk.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No risk exists with this id.", [payload.risk.header.id.0.clone()]);
    };
    if existing == &payload.risk {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This risk already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.risk).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.risk.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
