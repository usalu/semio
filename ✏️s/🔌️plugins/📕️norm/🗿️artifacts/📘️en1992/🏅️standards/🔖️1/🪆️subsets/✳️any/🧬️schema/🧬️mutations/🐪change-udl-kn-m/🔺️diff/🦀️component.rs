//! 🔺️ `change-udl-kn-m` sparse diff construction — writes only `En1992Diff.udl_kn_m` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_udl_kn_m::mutation::ChangeUdlKnM;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeUdlKnM, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_udl_kn_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Udl kn m must be a finite number.", Vec::<String>::new());
    }
    if base.udl_kn_m == payload.new_udl_kn_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Udl kn m already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { udl_kn_m: Some(payload.new_udl_kn_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
