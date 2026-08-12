//! 🔺️ `change-sfp-required-class` sparse diff construction — writes only `Din16798Diff.sfp_required_class` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_sfp_required_class::mutation::ChangeSfpRequiredClass;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSfpRequiredClass, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { sfp_required_class: Some(payload.new_sfp_required_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
