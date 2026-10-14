//! 🔺️ Sparse diff construction for the `create-register-row` leaf — handcrafted over the generated register catalog, never apply-then-capture.

use super::CreateRegisterRow;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists in the row's register, Error `mutation.target-missing` if `index` lies past its end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateRegisterRow, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let key = payload.row.key();
    let label = key.label();
    let id = &payload.row.header().id;
    if key.position(base, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists with this id.", label.sentence_subject()), [id.0.clone()]);
    }
    let length = key.len(base);
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("The index lies beyond the end of the {} list.", label.en), [id.0.clone()]);
    }
    protocol::MutationOutcome::new(payload.row.insertion(at))
}
