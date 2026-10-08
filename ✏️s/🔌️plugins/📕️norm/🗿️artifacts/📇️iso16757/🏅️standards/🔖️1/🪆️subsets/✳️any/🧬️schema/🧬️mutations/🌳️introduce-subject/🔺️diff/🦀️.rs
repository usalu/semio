//! 🔺️ `introduce-subject` — sparse diff construction.

use super::mutation::IntroduceSubject;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757SubjectsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an explicit index past the end is
/// `mutation.target-missing`.

pub fn diff(payload: &IntroduceSubject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A subject with id \"{}\" already exists.", payload.subject.id), [payload.subject.id.clone()]);
    }
    let len = base.dictionary.subjects.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.subject.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { subjects: Some(Iso16757SubjectsRows::insertion(payload.index.unwrap_or(len), payload.subject.clone())), ..Default::default() })
}
