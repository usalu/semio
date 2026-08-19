//! 🔺️ `change-anchor-f-uk-mpa` sparse diff construction — writes only `En1992Diff.anchor_f_uk_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_f_uk_mpa::mutation::ChangeAnchorFUkMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnchorFUkMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchor_f_uk_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Anchor f uk mpa must be a finite number.", Vec::<String>::new());
    }
    if base.anchor_f_uk_mpa == payload.new_anchor_f_uk_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Anchor f uk mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchor_f_uk_mpa: Some(payload.new_anchor_f_uk_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
