//! 🔺️ `change-h-tr-wk` sparse diff construction — writes only `Din16798Diff.h_tr_w_k` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_h_tr_w_k::mutation::ChangeHTrWK;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHTrWK, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { h_tr_w_k: Some(payload.new_h_tr_w_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
