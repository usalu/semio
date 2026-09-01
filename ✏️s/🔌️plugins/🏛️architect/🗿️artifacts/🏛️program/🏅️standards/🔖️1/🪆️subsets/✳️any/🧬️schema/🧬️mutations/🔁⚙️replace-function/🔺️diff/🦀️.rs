//! 🔺️ Sparse diff construction for the `replace-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::ReplaceFunction;
use crate::artifacts::program::diff::{ProgramFunctionsDelta, ProgramFunctionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceFunction, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.functions.iter().find(|row| row.header.id == payload.function.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No function exists with this id.", [payload.function.header.id.0.clone()]);
    };
    if existing == &payload.function {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This function already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.function).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { functions: Some(ProgramFunctionsDelta { patched: vec![ProgramFunctionsPatchEntry { id: payload.function.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
