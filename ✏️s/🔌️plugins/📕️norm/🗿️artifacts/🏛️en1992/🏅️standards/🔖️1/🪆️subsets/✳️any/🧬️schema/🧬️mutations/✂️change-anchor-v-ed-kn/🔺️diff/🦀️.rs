//! 🔺️ `change-anchor-v-ed-kn` sparse diff construction — writes only `En1992Diff.anchor_v_ed_kn` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_anchor_v_ed_kn::ChangeAnchorVEdKn;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorVEdKn, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_v_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor v ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_v_ed_kn == payload.new_anchor_v_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor v ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_v_ed_kn: Some(payload.new_anchor_v_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
