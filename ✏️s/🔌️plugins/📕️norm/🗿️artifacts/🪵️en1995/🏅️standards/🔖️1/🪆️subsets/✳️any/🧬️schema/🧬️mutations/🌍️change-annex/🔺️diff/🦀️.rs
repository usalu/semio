//! 🔺️ `change-annex` sparse diff construction — writes only `En1995Diff.annex` from the payload.

use crate::diff::En1995Diff;
use crate::mutations::set_snapshot::ChangeAnnex;
use crate::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
