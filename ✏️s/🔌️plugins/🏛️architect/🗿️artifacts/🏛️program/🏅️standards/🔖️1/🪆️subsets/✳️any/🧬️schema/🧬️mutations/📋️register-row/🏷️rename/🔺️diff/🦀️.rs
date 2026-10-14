//! 🔺️ Sparse diff construction for the `rename-register-row` leaf — handcrafted over the generated register catalog, never apply-then-capture.

use super::RenameRegisterRow;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🏷️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `modified = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameRegisterRow, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let label = payload.register.label();
    let Some(existing) = payload.register.position(base, &payload.id).and_then(|position| payload.register.row_at(base, position)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No {} exists with this id.", label.en), [payload.id.0.clone()]);
    };
    if existing.header().name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("This {} already has this name.", label.en)).at([payload.id.0.clone()])]);
    }
    protocol::MutationOutcome::new(payload.register.renaming(&payload.id, payload.new_name.clone()))
}
