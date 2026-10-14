//! ↩️ `update-ventilation` inverse.

use crate::update_ventilation::UpdateVentilation;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &UpdateVentilation, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_ventilation::UpdateVentilation> {
    Ok((|| {
    vec![Op::from(UpdateVentilation { new_ventilation: base.ventilation.clone() })]

    })())
}
