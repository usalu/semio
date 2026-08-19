//! 🔺️ `change-annex` sparse diff construction — writes only `En1995Diff.annex` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnnex, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
