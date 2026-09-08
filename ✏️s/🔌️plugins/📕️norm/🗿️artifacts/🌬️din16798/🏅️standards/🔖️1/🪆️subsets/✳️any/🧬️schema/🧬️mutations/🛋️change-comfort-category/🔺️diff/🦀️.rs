//! 🔺️ `change-comfort-category` sparse diff construction — writes only `Din16798Diff.comfort_category` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_comfort_category::ChangeComfortCategory;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeComfortCategory, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.comfort_category == payload.new_comfort_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Comfort category is already \"{}\".", payload.new_comfort_category));
    }
    protocol::MutationOutcome::new(Din16798Diff { comfort_category: Some(payload.new_comfort_category.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
