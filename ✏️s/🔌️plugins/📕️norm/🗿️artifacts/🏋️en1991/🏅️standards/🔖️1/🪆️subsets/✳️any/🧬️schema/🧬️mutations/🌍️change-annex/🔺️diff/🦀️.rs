//! 🔺️ `change-annex` — sparse diff construction.

use super::ChangeAnnex;
use crate::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
