//! 🔺️ Sparse diff construction for the `replace-register-row` leaf — handcrafted over the generated register catalog, never apply-then-capture.

use super::ReplaceRegisterRow;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ♻️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceRegisterRow, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let key = payload.row.key();
    let label = key.label();
    let id = &payload.row.header().id;
    let Some(position) = key.position(base, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No {} exists with this id.", label.en), [id.0.clone()]);
    };
    if payload.row.equals_row_at(base, position) {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("This {} already matches the requested value.", label.en)).at([id.0.clone()])]);
    }
    protocol::MutationOutcome::new(payload.row.replacement(base, position))
}
