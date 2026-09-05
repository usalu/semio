//! 🔺️ `change-annex` sparse diff construction — writes only `Din16798Diff.annex` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_annex::ChangeAnnex;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("National annex is already {:?}.", payload.new_annex));
    }
    protocol::MutationOutcome::new(Din16798Diff { annex: Some(payload.new_annex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
