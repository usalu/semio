//! ↩️ `change-has-mechanical-ventilation` inverse.

use crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeHasMechanicalVentilation, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation> {
    Ok((|| {
    vec![Op::from(ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: base.has_mechanical_ventilation })]

    })())
}
