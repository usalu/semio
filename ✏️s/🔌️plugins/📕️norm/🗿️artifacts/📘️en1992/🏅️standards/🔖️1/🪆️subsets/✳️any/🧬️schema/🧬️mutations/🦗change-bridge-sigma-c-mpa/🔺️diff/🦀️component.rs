//! 🔺️ `change-bridge-sigma-c-mpa` sparse diff construction — writes only `En1992Diff.bridge_sigma_c_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_bridge_sigma_c_mpa::mutation::ChangeBridgeSigmaCMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBridgeSigmaCMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_bridge_sigma_c_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bridge sigma c mpa must be a finite number.", Vec::<String>::new());
    }
    if base.bridge_sigma_c_mpa == payload.new_bridge_sigma_c_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bridge sigma c mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { bridge_sigma_c_mpa: Some(payload.new_bridge_sigma_c_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
