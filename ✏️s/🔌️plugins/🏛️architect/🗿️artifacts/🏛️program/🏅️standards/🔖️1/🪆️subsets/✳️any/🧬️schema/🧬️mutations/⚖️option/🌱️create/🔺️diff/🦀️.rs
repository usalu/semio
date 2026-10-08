//! 🔺️ Sparse diff construction for the `create-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::CreateOptionEvaluation;
use crate::diff::ProgramOptionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateOptionEvaluation, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.option_evaluation.header.id;
    if base.options.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An option evaluation already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.options.len());
    if at > base.options.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the option evaluation list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { options: Some(ProgramOptionsDelta::insertion(at, payload.option_evaluation.clone())), ..Default::default() })
}
