//! 🔺️ `change-annex` sparse diff construction — writes only `En1998Diff.annex` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_annex::ChangeAnnex;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already \"{}\".", payload.new_annex));
    }
    protocol::MutationOutcome::new(En1998Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
