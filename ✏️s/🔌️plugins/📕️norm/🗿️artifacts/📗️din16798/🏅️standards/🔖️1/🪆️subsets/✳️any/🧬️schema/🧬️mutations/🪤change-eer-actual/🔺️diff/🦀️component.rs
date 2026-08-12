//! 🔺️ `change-eer-actual` sparse diff construction — writes only `Din16798Diff.eer_actual` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_eer_actual::mutation::ChangeEerActual;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEerActual, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { eer_actual: Some(payload.new_eer_actual.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
