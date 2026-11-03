//! 🔺️ `change-fire-rating` diff: patches `rating` of the addressed fire exposure row.

use crate::change_fire_rating::ChangeFireRating;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FireExposureDelta, En1993FireExposurePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFireRating, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fire_exposures.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-exposure", Vec::<String>::new());
    };
    if row.rating == payload.new_rating {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire exposure already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: En1993FireExposureDelta::modification(payload.id.clone(), En1993FireExposurePatch { rating: Some(payload.new_rating), ..Default::default() }), ..Default::default() })
}
