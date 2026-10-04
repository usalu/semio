//! ↩️ `update-ventilation` inverse.

use crate::mutations::update_ventilation::UpdateVentilation;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &UpdateVentilation, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::UpdateVentilation(UpdateVentilation { new_ventilation: base.ventilation.clone() })]

    })())
}
