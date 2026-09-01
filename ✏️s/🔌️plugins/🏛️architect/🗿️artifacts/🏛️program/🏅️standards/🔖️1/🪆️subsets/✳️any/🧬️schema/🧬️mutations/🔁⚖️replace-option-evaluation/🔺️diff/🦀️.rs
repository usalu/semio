//! 🔺️ Sparse diff construction for the `replace-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::ReplaceOptionEvaluation;
use crate::artifacts::program::diff::{ProgramOptionsDelta, ProgramOptionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceOptionEvaluation, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No option evaluation exists with this id.", [payload.option_evaluation.header.id.0.clone()]);
    };
    if existing == &payload.option_evaluation {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This option evaluation already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.option_evaluation).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { options: Some(ProgramOptionsDelta { patched: vec![ProgramOptionsPatchEntry { id: payload.option_evaluation.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
