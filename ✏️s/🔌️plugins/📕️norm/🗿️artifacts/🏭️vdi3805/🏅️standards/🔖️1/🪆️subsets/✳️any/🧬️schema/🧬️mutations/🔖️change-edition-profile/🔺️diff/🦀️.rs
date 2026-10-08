//! 🔺️ `change-edition-profile` — sparse diff construction.

use super::ChangeEditionProfile;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805EditionProfileRows, Vdi3805EditionProfileEntry};

//#region 🔖️Diff

pub fn diff(payload: &ChangeEditionProfile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.edition_profile.get(&payload.sheet) == Some(&payload.new_choice) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Sheet {} already has this edition profile.", payload.sheet));
    }
    let entry = Vdi3805EditionProfileEntry { key: payload.sheet.clone(), value: payload.new_choice };
    let rows = if base.edition_profile.contains_key(&payload.sheet) {
        Vdi3805EditionProfileRows { modified: vec![entry], ..Default::default() }
    } else {
        Vdi3805EditionProfileRows { added: vec![entry], ..Default::default() }
    };
    protocol::MutationOutcome::new(Vdi3805Diff { edition_profile: Some(rows), ..Default::default() })
}
