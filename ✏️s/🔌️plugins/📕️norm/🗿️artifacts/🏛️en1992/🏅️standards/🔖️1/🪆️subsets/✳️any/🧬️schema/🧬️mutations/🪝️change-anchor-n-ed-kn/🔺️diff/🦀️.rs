//! 🔺️ `change-anchor-n-ed-kn` sparse diff construction — writes only `En1992Diff.anchor_n_ed_kn` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_anchor_n_ed_kn::ChangeAnchorNEdKn;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorNEdKn, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor n ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_n_ed_kn == payload.new_anchor_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor n ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_n_ed_kn: Some(payload.new_anchor_n_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
