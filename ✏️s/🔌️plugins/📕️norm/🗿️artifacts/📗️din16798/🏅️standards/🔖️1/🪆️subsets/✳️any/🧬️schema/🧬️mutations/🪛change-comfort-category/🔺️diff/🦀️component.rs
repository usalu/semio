//! 🔺️ `change-comfort-category` sparse diff construction — writes only `Din16798Diff.comfort_category` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_comfort_category::mutation::ChangeComfortCategory;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeComfortCategory, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.comfort_category == payload.new_comfort_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Comfort category is already \"{}\".", payload.new_comfort_category));
    }
    protocol::MutationOutcome::new(Din16798Diff { comfort_category: Some(payload.new_comfort_category.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
