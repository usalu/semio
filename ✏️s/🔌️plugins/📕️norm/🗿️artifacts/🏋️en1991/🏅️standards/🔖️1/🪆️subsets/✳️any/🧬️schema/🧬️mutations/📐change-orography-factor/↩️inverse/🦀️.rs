//! Inverse for `change-orography-factor`.
use crate::change_orography_factor::ChangeOrographyFactor;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeOrographyFactor, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_orography_factor::ChangeOrographyFactor> {
    Ok((|| {
    vec![Op::from(ChangeOrographyFactor { new_orography_factor: base.orography_factor })]

    })())
}
