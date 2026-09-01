//! 🔺️ `change-annex` sparse diff construction — writes only `En1997Diff.annex` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_annex::ChangeAnnex;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already {:?}.", payload.new_annex));
    }
    protocol::MutationOutcome::new(En1997Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
