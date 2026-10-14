//! ↩️ `specify-heating-system` inverse.

use crate::specify_heating_system::SpecifyHeatingSystem;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &SpecifyHeatingSystem, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::specify_heating_system::SpecifyHeatingSystem> {
    Ok((|| {
    vec![Op::from(SpecifyHeatingSystem { new_heating: base.heating.clone() })]

    })())
}
