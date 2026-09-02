//! 🔺️ `create-subject` — sparse diff construction.

use super::mutation::CreateSubject;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.
pub fn diff(payload: &CreateSubject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A subject with id \"{}\" already exists.", payload.subject.id), [payload.subject.id.clone()]);
    }
    let mut dictionary = base.dictionary.clone();
    let clamped = matches!(payload.index, Some(index) if index > dictionary.subjects.len());
    match payload.index {
        Some(index) if index <= dictionary.subjects.len() => dictionary.subjects.insert(index, payload.subject.clone()),
        _ => dictionary.subjects.push(payload.subject.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { dictionary: Some(dictionary), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index was out of range; appended subject \"{}\" at the end instead.", payload.subject.id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
