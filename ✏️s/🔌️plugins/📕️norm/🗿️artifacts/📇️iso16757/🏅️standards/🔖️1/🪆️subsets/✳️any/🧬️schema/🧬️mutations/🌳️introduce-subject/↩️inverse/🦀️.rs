//! ↩️ `introduce-subject` — undo is `retire-subject`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::mutations::retire_subject;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::IntroduceSubject;

//#region 🔖️Inverse
pub fn inverse(payload: &IntroduceSubject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireSubject(retire_subject::mutation::RetireSubject { id: payload.subject.id.clone() })]
}
//#endregion 🔖️Inverse
