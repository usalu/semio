//! 🔺️ Sparse diff construction for the `delete-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::DeleteFunction;
use crate::standards::v1::subsets::any::schema::diff::ProgramFunctionsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteFunction, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.functions.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No function exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { functions: Some(ProgramFunctionsDelta::removal(&base.functions, position)), ..Default::default() })
}
