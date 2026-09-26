//! Diff for `change-fire-rating`.
use super::ChangeFireRating;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeFireRating, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.new_fire_rating.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.fire_rating == payload.new_fire_rating {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() })
}
