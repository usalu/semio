//! 🔺️ `change-annex` — sparse diff construction.

use super::mutation::ChangeAnnex;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAnnex, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
