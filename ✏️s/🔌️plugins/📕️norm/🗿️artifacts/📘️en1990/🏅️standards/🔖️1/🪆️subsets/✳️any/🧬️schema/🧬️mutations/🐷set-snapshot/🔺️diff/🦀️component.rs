//! 🔺️ `change-annex` sparse diff construction — writes only `En1990Diff.annex` from the payload.

use crate::artifacts::en1990::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnnex, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
