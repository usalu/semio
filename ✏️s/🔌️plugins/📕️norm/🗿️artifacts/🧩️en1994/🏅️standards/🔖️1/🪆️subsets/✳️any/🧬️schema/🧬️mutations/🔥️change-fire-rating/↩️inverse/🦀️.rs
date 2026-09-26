//! Inverse for `change-fire-rating`.
use super::ChangeFireRating;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeFireRating, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: base.fire_rating.clone() })]
}
