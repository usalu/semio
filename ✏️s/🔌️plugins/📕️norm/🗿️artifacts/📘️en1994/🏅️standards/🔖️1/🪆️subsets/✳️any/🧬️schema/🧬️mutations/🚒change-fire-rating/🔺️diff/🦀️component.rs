//! 🔺️ `change-fire-rating` — sparse diff construction.

use super::mutation::ChangeFireRating;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireRating, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
