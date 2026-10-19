//! ↩️ Inverse for `change-silo-hydraulic-radius`.
use crate::change_silo_hydraulic_radius::ChangeSiloHydraulicRadius;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloHydraulicRadius, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_hydraulic_radius::ChangeSiloHydraulicRadius> {
    Ok((|| {
    vec![Op::from(ChangeSiloHydraulicRadius { new_silo_hydraulic_radius: base.silo_hydraulic_radius })]

    })())
}
