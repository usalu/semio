//! ↩️ `delete-subject` — undo re-`create`s the subject from BASE state, at its original index;
//! missing id ⇒ `Vec::new()`.

use crate::artifacts::iso16757::mutations::create_subject;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::DeleteSubject;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSubject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.dictionary.subjects.iter().position(|subject| subject.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::CreateSubject(create_subject::mutation::CreateSubject { subject: base.dictionary.subjects[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
