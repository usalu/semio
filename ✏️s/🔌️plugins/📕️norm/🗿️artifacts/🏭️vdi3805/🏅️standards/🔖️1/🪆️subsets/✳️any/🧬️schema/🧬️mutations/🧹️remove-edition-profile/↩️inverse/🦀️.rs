//! ↩️ `remove-edition-profile` — undo restores the BASE value via `change`; missing sheet ⇒
//! `Vec::new()`.

use crate::remove_edition_profile::RemoveEditionProfile;
use crate::change_edition_profile;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveEditionProfile, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_edition_profile::ChangeEditionProfile> {
    Ok((|| {
    match base.edition_profile.get(&payload.sheet) {
        Some(old_choice) => vec![Op::from(change_edition_profile::ChangeEditionProfile { sheet: payload.sheet.clone(), new_choice: *old_choice })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
