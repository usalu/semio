//! 🔺️ `change-annex` sparse diff construction — writes only `Din16798Diff.annex` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_annex::ChangeAnnex;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already {:?}.", payload.new_annex));
    }
    protocol::MutationOutcome::new(Din16798Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
