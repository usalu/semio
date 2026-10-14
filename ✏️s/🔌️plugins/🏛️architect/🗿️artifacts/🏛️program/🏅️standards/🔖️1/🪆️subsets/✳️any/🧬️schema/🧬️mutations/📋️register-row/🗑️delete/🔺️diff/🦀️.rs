//! 🔺️ Sparse diff construction for the `delete-register-row` leaf — handcrafted over the generated register catalog, never apply-then-capture.

use super::DeleteRegisterRow;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent from the register (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteRegisterRow, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = payload.register.position(base, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No {} exists with this id.", payload.register.label().en), [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(payload.register.removal(base, position))
}
