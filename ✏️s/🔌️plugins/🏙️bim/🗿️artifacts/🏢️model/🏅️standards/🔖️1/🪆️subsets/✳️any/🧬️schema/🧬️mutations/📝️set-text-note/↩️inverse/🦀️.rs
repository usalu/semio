//! ↩️ Inverse of `SetTextNote`: an absolute `SetTextNote` restoring the base value of exactly the fields the forward really changes, none when the text note is absent or nothing changes.

use super::SetTextNote;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetTextNote, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.text_notes.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetTextNote(SetTextNote::from_patch(payload.id.clone(), restore))]
}
