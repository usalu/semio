//! 🔺️ `change-anchor-f-yk-mpa` sparse diff construction — writes only `En1992Diff.anchor_f_yk_mpa` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_anchor_f_yk_mpa::ChangeAnchorFYkMpa;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorFYkMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_f_yk_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor f yk mpa must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_f_yk_mpa == payload.new_anchor_f_yk_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor f yk mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_f_yk_mpa: Some(payload.new_anchor_f_yk_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
