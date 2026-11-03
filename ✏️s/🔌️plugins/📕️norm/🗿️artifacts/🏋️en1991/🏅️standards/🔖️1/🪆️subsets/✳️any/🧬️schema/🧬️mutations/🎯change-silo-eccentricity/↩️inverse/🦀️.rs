//! ↩️ Inverse for `change-silo-eccentricity`.
use crate::change_silo_eccentricity::ChangeSiloEccentricity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloEccentricity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_eccentricity::ChangeSiloEccentricity> {
    Ok((|| {
    vec![Op::from(ChangeSiloEccentricity { new_silo_eccentricity: base.silo_eccentricity })]

    })())
}
