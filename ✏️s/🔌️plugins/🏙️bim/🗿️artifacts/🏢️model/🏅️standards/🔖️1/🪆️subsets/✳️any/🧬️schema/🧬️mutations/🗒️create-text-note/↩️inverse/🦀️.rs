//! ↩️ Inverse of `CreateTextNote`: the concrete `DeleteTextNote` of the id it created, none when the id was already taken.

use super::super::delete_text_note::DeleteTextNote;
use super::CreateTextNote;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateTextNote, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.text_notes.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteTextNote(DeleteTextNote { id: payload.id.clone() })]
}
