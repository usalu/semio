//! ↩️ `change-edition-profile` — undo restores the BASE value, or `remove`s the override if it was
//! previously absent (this mutation upserts, so a fresh sheet's undo is `remove`, not `change`).

use crate::change_edition_profile::ChangeEditionProfile;
use crate::remove_edition_profile;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeEditionProfile, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_edition_profile::ChangeEditionProfile> + From<crate::remove_edition_profile::RemoveEditionProfile> {
    Ok((|| {
    match base.edition_profile.get(&payload.sheet) {
        Some(old_choice) => vec![Op::from(ChangeEditionProfile { sheet: payload.sheet.clone(), new_choice: *old_choice })],
        None => vec![Op::from(remove_edition_profile::RemoveEditionProfile { sheet: payload.sheet.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
