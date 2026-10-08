//! 🔺️ `introduce-subject` — sparse diff construction.

use super::mutation::IntroduceSubject;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757SubjectsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.

pub fn diff(payload: &IntroduceSubject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A subject with id \"{}\" already exists.", payload.subject.id), [payload.subject.id.clone()]);
    }
    let ids: Vec<String> = base.dictionary.subjects.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.subject.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { subjects: Some(Iso16757SubjectsRows { added: vec![payload.subject.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended subject \"{}\" at the end instead.", payload.subject.id))
    } else {
        outcome
    }
}
