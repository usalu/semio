//! 🔺️ `change-n50-h-inv` sparse diff construction — writes only `Din16798Diff.n50_h_inv` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_n50_h_inv::mutation::ChangeN50HInv;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeN50HInv, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { n50_h_inv: Some(payload.new_n50_h_inv.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
