//! 🔺️ `change-annex` sparse diff construction — writes only `Din16798Diff.annex` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_annex::mutation::ChangeAnnex;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
