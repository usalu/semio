//! ↩️ `update-renewables` inverse.

use crate::update_renewables::UpdateRenewables;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &UpdateRenewables, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_renewables::UpdateRenewables> {
    Ok((|| {
    vec![Op::from(UpdateRenewables { new_renewables: base.renewables.clone() })]

    })())
}
