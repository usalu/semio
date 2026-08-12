//! 🔺️ `change-fire-rating` sparse diff construction — writes only `En1992Diff.fire_rating` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_fire_rating::mutation::ChangeFireRating;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireRating, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
