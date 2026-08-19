//! 🔺️ `change-annex` sparse diff construction — writes only `En1999Diff.annex` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnnex, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already {:?}.", payload.new_annex));
    }
    protocol::MutationOutcome::new(En1999Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
