//! ↩️ `update-renewables` inverse.

use crate::mutations::update_renewables::UpdateRenewables;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &UpdateRenewables, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::UpdateRenewables(UpdateRenewables { new_renewables: base.renewables.clone() })]

    })())
}
