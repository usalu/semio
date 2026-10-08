//! 🔺️ Sparse diff construction for the `replace-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::ReplaceOptionEvaluation;
use crate::diff::ProgramOptionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceOptionEvaluation, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.option_evaluation.header.id;
    let Some(position) = base.options.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No option evaluation exists with this id.", [id.0.clone()]);
    };
    if base.options[position] == payload.option_evaluation {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This option evaluation already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramOptionsDelta::removal(&base.options, position);
    delta.absorb(ProgramOptionsDelta::insertion(position, payload.option_evaluation.clone()));
    protocol::MutationOutcome::new(ProgramDiff { options: Some(delta), ..Default::default() })
}
