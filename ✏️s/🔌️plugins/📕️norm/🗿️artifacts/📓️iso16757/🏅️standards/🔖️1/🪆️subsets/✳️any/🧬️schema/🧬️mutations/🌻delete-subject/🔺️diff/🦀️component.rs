//! 🔺️ `delete-subject` — sparse diff construction.

use super::mutation::DeleteSubject;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &DeleteSubject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.dictionary.subjects.iter().any(|subject| subject.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Subject \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut dictionary = base.dictionary.clone();
    dictionary.subjects.retain(|subject| subject.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { dictionary: Some(dictionary), ..Default::default() })
}
//#endregion 🔖️Diff
