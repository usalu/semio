//! Inverse for `change-fire-rating`.
use crate::change_fire_rating::ChangeFireRating;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireRating, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_rating::ChangeFireRating> {
    Ok((|| {
    vec![Op::from(ChangeFireRating { new_fire_rating: base.fire_rating.clone() })]

    })())
}
