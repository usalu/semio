//! 🔺️ `create-subject` — sparse diff construction.

use super::mutation::CreateSubject;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the dictionary clone is returned unchanged rather than pushing a second subject.
pub fn diff(payload: &CreateSubject, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut dictionary = base.dictionary.clone();
    if !dictionary.subjects.iter().any(|subject| subject.id == payload.subject.id) {
        match payload.index {
            Some(index) if index <= dictionary.subjects.len() => dictionary.subjects.insert(index, payload.subject.clone()),
            _ => dictionary.subjects.push(payload.subject.clone()),
        }
    }
    Iso16757Diff { dictionary: Some(dictionary), ..Default::default() }
}
//#endregion 🔖️Diff
