//! 🔺️ Diff constructor for `DeleteTextNote`: the text note leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use super::super::cascade;
use super::DeleteTextNote;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteTextNote, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.text_notes.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Text note \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Text note", Some(&payload.id))
}
