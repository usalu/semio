//! 🔺️ `change-pile-n-profiles` sparse diff construction — writes only `En1997Diff.pile_n_profiles` from the payload.

use crate::diff::En1997Diff;
use crate::mutations::change_pile_n_profiles::ChangePileNProfiles;
use crate::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileNProfiles, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.pile_n_profiles == payload.new_pile_n_profiles {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of investigated pile profiles is already {}.", payload.new_pile_n_profiles));
    }
    protocol::MutationOutcome::new(En1997Diff { pile_n_profiles: Some(payload.new_pile_n_profiles), ..Default::default() })
}
//#endregion 🔖️Diff
