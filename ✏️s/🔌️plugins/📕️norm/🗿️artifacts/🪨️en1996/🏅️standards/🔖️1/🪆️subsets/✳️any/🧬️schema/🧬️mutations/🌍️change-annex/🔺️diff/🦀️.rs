//! 🔺️ `change-annex` sparse diff construction — writes only `En1996Diff.annex` from the payload.

use crate::diff::En1996Diff;
use crate::mutations::change_annex::ChangeAnnex;
use crate::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
