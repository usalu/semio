//! ↩️ `change-fire-rating` — undo restores BASE's fire_rating.

use super::ChangeFireRating;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireRating, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: base.fire_rating.clone() })]
}
//#endregion 🔖️Inverse
