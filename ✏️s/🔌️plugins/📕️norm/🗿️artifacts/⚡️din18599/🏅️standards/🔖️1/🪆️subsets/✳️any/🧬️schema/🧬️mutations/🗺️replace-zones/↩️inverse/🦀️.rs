//! ↩️ `replace-zones` inverse.

use crate::replace_zones::ReplaceZones;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ReplaceZones, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::replace_zones::ReplaceZones> {
    Ok((|| {
    vec![Op::from(ReplaceZones { new_zones: base.zones.clone() })]

    })())
}
