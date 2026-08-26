//! ↩️ `create-subject` — undo is `delete-subject`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::artifacts::iso16757::mutations::delete_subject;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::CreateSubject;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSubject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::DeleteSubject(delete_subject::mutation::DeleteSubject { id: payload.subject.id.clone() })]
}
//#endregion 🔖️Inverse
