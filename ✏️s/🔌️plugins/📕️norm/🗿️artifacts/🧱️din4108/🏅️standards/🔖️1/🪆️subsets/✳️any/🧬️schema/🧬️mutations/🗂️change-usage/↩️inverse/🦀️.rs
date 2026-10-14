//! ↩️ `change-usage` inverse.

use crate::change_usage::ChangeUsage;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeUsage, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_usage::ChangeUsage> {
    Ok((|| {
    vec![Op::from(ChangeUsage { new_usage: base.usage.clone() })]

    })())
}
