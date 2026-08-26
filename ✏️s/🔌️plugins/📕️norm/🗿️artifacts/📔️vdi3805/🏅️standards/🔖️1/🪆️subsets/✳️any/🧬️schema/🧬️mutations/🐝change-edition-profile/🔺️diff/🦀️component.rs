//! 🔺️ `change-edition-profile` — sparse diff construction.

use super::mutation::ChangeEditionProfile;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEditionProfile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.edition_profile.get(&payload.sheet) == Some(&payload.new_choice) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sheet {} already has this edition profile.", payload.sheet));
    }
    let mut edition_profile = base.edition_profile.clone();
    edition_profile.insert(payload.sheet.clone(), payload.new_choice);
    protocol::MutationOutcome::new(Vdi3805Diff { edition_profile: Some(edition_profile), ..Default::default() })
}
//#endregion 🔖️Diff
