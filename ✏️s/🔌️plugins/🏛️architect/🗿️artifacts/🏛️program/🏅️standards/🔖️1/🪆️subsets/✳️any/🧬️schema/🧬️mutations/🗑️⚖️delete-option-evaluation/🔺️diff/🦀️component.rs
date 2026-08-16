//! 🔺️ Sparse diff construction for the `delete-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::mutation::DeleteOptionEvaluation;
use crate::artifacts::program::diff::ProgramOptionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteOptionEvaluation, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.options.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No option evaluation exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { options: Some(ProgramOptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
