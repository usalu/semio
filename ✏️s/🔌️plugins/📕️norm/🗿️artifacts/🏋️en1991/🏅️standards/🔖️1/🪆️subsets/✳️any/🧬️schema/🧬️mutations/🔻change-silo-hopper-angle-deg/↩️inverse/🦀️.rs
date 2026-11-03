//! ↩️ Inverse for `change-silo-hopper-angle-deg`.
use crate::change_silo_hopper_angle_deg::ChangeSiloHopperAngleDeg;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeSiloHopperAngleDeg, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_hopper_angle_deg::ChangeSiloHopperAngleDeg> {
    Ok((|| {
    vec![Op::from(ChangeSiloHopperAngleDeg { new_silo_hopper_angle_deg: base.silo_hopper_angle_deg })]

    })())
}
