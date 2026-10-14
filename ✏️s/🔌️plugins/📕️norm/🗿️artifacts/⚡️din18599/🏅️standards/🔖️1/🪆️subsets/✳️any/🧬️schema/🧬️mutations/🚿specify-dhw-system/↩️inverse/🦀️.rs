//! ↩️ `specify-dhw-system` inverse.

use crate::specify_dhw_system::SpecifyDhwSystem;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &SpecifyDhwSystem, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::specify_dhw_system::SpecifyDhwSystem> {
    Ok((|| {
    vec![Op::from(SpecifyDhwSystem { new_dhw: base.dhw.clone() })]

    })())
}
