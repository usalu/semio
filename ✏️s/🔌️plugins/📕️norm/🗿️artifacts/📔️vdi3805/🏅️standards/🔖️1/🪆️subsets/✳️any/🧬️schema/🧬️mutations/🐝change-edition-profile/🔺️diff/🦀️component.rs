//! 🔺️ `change-edition-profile` — sparse diff construction.

use super::mutation::ChangeEditionProfile;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEditionProfile, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut edition_profile = base.edition_profile.clone();
    edition_profile.insert(payload.sheet.clone(), payload.new_choice);
    Vdi3805Diff { edition_profile: Some(edition_profile), ..Default::default() }
}
//#endregion 🔖️Diff
