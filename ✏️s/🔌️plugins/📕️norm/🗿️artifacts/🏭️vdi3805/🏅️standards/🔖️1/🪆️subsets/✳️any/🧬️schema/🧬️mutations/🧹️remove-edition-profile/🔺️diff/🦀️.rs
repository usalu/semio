//! 🔺️ `remove-edition-profile` — sparse diff construction.

use crate::remove_edition_profile::RemoveEditionProfile;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_s_artifact_norm_vdi3805_diff::{Vdi3805Diff, Vdi3805EditionProfileRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveEditionProfile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.edition_profile.contains_key(&payload.sheet) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Sheet {} has no edition profile override.", payload.sheet), [payload.sheet.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { edition_profile: Some(Vdi3805EditionProfileRows { removed: vec![payload.sheet.clone()], ..Default::default() }), ..Default::default() })
}
