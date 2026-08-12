//! ↩️ `remove-edition-profile` — undo restores the BASE value via `change`; missing sheet ⇒
//! `Vec::new()`.

use super::mutation::RemoveEditionProfile;
use crate::artifacts::vdi3805::mutations::change_edition_profile;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveEditionProfile, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.edition_profile.get(&payload.sheet) {
        Some(old_choice) => vec![Vdi3805Mutation::ChangeEditionProfile(change_edition_profile::mutation::ChangeEditionProfile { sheet: payload.sheet.clone(), new_choice: *old_choice })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
