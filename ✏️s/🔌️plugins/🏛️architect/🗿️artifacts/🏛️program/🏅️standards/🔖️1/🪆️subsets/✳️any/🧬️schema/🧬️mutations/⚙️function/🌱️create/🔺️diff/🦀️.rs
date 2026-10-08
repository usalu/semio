//! 🔺️ Sparse diff construction for the `create-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::CreateFunction;
use crate::diff::ProgramFunctionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateFunction, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.function.header.id;
    if base.functions.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A function already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.functions.len());
    if at > base.functions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the function list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { functions: Some(ProgramFunctionsDelta::insertion(at, payload.function.clone())), ..Default::default() })
}
