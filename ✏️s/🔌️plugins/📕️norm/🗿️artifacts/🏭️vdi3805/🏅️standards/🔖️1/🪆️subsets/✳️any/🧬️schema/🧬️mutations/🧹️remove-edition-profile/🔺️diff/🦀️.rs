//! 🔺️ `remove-edition-profile` — sparse diff construction.

use super::RemoveEditionProfile;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805EditionProfileRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveEditionProfile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.edition_profile.contains_key(&payload.sheet) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Sheet {} has no edition profile override.", payload.sheet), [payload.sheet.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { edition_profile: Some(Vdi3805EditionProfileRows { removed: vec![payload.sheet.clone()], ..Default::default() }), ..Default::default() })
}
