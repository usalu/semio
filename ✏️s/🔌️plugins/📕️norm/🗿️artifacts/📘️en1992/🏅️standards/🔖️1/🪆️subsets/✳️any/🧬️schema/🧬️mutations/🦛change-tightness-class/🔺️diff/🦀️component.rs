//! 🔺️ `change-tightness-class` sparse diff construction — writes only `En1992Diff.tightness_class` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_tightness_class::mutation::ChangeTightnessClass;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTightnessClass, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { tightness_class: Some(payload.new_tightness_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
