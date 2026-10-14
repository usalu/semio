//! Inverse for `change-fire-compartment-area`.
use crate::change_fire_compartment_area::ChangeFireCompartmentArea;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeFireCompartmentArea, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_compartment_area::ChangeFireCompartmentArea> {
    Ok((|| {
    vec![Op::from(ChangeFireCompartmentArea { new_fire_compartment_area: base.fire_compartment_area })]

    })())
}
