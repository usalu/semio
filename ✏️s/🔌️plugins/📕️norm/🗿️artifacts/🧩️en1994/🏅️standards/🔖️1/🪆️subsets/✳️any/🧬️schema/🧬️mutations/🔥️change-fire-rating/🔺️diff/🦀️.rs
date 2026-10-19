//! 🔺️ Diff for `change-fire-rating`.
use crate::change_fire_rating::ChangeFireRating;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeFireRating, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.new_fire_rating.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.fire_rating == payload.new_fire_rating {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { fire_rating: Some(payload.new_fire_rating.clone()), ..Default::default() })
}
