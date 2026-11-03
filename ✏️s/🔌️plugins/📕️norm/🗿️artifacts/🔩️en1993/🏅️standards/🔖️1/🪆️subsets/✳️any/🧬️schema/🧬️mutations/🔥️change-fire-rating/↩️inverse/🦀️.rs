//! ↩️ `change-fire-rating` inverse: restores the previous `rating`.

use crate::change_fire_rating::ChangeFireRating;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireRating, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_rating::ChangeFireRating> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireRating { id: payload.id.clone(), new_rating: row.rating })],
        None => Vec::new(),
    })
}
