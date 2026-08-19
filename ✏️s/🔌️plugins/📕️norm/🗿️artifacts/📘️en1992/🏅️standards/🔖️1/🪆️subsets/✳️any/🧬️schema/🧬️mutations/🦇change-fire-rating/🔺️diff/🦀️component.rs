//! 🔺️ `change-fire-rating` sparse diff construction — writes only `En1992Diff.fire_rating` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_fire_rating::mutation::ChangeFireRating;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFireRating, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.fire_rating == payload.new_fire_rating {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire rating already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
