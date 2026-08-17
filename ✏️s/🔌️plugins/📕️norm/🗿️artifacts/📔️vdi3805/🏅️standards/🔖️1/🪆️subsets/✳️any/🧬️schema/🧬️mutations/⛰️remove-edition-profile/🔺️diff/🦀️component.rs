//! 🔺️ `remove-edition-profile` — sparse diff construction.

use super::mutation::RemoveEditionProfile;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveEditionProfile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.edition_profile.contains_key(&payload.sheet) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Sheet {} has no edition profile override.", payload.sheet), [payload.sheet.clone()]);
    }
    let mut edition_profile = base.edition_profile.clone();
    edition_profile.remove(&payload.sheet);
    protocol::MutationOutcome::new(Vdi3805Diff { edition_profile: Some(edition_profile), ..Default::default() })
}
//#endregion 🔖️Diff
