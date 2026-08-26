//! 🔺️ `change-fire-rating` — sparse diff construction.

use super::mutation::ChangeFireRating;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireRating, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.fire_rating == payload.new_fire_rating {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire rating already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
