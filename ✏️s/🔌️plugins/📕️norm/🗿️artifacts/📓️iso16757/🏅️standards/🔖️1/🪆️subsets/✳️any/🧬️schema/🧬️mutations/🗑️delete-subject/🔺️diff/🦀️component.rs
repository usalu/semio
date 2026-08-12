//! 🔺️ `delete-subject` — sparse diff construction.

use super::mutation::DeleteSubject;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteSubject, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut dictionary = base.dictionary.clone();
    dictionary.subjects.retain(|subject| subject.id != payload.id);
    Iso16757Diff { dictionary: Some(dictionary), ..Default::default() }
}
//#endregion 🔖️Diff
