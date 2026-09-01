//! 🔺️ `change-annex` sparse diff construction — writes only `En1998Diff.annex` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_annex::ChangeAnnex;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already \"{}\".", payload.new_annex));
    }
    protocol::MutationOutcome::new(En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
