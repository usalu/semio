//! 🔺️ `change-duct-class` sparse diff construction — writes only `Din16798Diff.duct_class` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_duct_class::mutation::ChangeDuctClass;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDuctClass, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.duct_class == payload.new_duct_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Duct leakage class is already \"{}\".", payload.new_duct_class));
    }
    protocol::MutationOutcome::new(Din16798Diff { duct_class: Some(payload.new_duct_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
