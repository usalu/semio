//! 🔺️ `change-annex` — sparse diff construction.

use super::ChangeAnnex;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
