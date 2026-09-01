//! 🔺️ `change-bedrooms` sparse diff construction — writes only `Din16798Diff.bedrooms` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_bedrooms::ChangeBedrooms;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBedrooms, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.bedrooms == payload.new_bedrooms {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of bedrooms is already {}.", payload.new_bedrooms));
    }
    protocol::MutationOutcome::new(Din16798Diff { bedrooms: Some(payload.new_bedrooms.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
