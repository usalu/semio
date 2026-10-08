//! 🔺️ `retire-subject` — sparse diff construction.

use super::mutation::RetireSubject;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757SubjectsRows};

//#region 🔖️Diff

pub fn diff(payload: &RetireSubject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.dictionary.subjects.iter().any(|subject| subject.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Subject \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Iso16757Diff { subjects: Some(Iso16757SubjectsRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
