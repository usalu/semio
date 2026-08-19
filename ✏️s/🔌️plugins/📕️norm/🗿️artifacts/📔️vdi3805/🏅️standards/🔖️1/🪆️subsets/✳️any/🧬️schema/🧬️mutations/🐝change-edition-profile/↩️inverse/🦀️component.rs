//! ↩️ `change-edition-profile` — undo restores the BASE value, or `remove`s the override if it was
//! previously absent (this mutation upserts, so a fresh sheet's undo is `remove`, not `change`).

use super::mutation::ChangeEditionProfile;
use crate::artifacts::vdi3805::mutations::remove_edition_profile;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeEditionProfile, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.edition_profile.get(&payload.sheet) {
        Some(old_choice) => vec![Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: payload.sheet.clone(), new_choice: *old_choice })],
        None => vec![Vdi3805Mutation::RemoveEditionProfile(remove_edition_profile::mutation::RemoveEditionProfile { sheet: payload.sheet.clone() })],
    }
}
//#endregion 🔖️Inverse
