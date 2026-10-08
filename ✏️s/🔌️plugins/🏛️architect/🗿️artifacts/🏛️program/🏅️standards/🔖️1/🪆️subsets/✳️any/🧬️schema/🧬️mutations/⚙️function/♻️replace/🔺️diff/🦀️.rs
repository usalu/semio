//! 🔺️ Sparse diff construction for the `replace-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::ReplaceFunction;
use crate::diff::ProgramFunctionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceFunction, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.function.header.id;
    let Some(position) = base.functions.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No function exists with this id.", [id.0.clone()]);
    };
    if base.functions[position] == payload.function {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This function already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramFunctionsDelta::removal(&base.functions, position);
    delta.absorb(ProgramFunctionsDelta::insertion(position, payload.function.clone()));
    protocol::MutationOutcome::new(ProgramDiff { functions: Some(delta), ..Default::default() })
}
