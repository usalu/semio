//! ↩️ `replace-zones` inverse.

use crate::mutations::replace_zones::ReplaceZones;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ReplaceZones, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::ReplaceZones(ReplaceZones { new_zones: base.zones.clone() })]

    })())
}
