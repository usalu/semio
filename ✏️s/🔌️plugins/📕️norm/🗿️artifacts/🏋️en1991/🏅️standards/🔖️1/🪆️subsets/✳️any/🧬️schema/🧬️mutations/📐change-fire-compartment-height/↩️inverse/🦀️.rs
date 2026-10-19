//! ↩️ Inverse for `change-fire-compartment-height`.
use crate::change_fire_compartment_height::ChangeFireCompartmentHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireCompartmentHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_compartment_height::ChangeFireCompartmentHeight> {
    Ok((|| {
    vec![Op::from(ChangeFireCompartmentHeight { new_fire_compartment_height: base.fire_compartment_height })]

    })())
}
