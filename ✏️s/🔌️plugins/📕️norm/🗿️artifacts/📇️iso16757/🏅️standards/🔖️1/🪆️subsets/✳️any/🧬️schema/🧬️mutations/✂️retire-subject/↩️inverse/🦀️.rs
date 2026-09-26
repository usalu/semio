//! ↩️ `retire-subject` — undo re-`create`s the subject from BASE state, at its original index;
//! missing id ⇒ `Vec::new()`.

use crate::mutations::introduce_subject;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RetireSubject;

//#region 🔖️Inverse
pub fn inverse(payload: &RetireSubject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.dictionary.subjects.iter().position(|subject| subject.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceSubject(introduce_subject::mutation::IntroduceSubject { subject: base.dictionary.subjects[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
