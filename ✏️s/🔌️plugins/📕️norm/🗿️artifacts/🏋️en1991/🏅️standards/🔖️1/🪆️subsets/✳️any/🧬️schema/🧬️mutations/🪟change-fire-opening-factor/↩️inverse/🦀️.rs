//! Inverse for `change-fire-opening-factor`.
use crate::change_fire_opening_factor::ChangeFireOpeningFactor;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireOpeningFactor, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_opening_factor::ChangeFireOpeningFactor> {
    Ok((|| {
    vec![Op::from(ChangeFireOpeningFactor { new_fire_opening_factor: base.fire_opening_factor })]

    })())
}
