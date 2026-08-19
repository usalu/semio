//! 🔺️ `change-rho-l` sparse diff construction — writes only `En1992Diff.rho_l` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_rho_l::mutation::ChangeRhoL;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeRhoL, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_rho_l.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Rho l must be a finite number.", Vec::<String>::new());
    }
    if base.rho_l == payload.new_rho_l {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Rho l already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { rho_l: Some(payload.new_rho_l.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
