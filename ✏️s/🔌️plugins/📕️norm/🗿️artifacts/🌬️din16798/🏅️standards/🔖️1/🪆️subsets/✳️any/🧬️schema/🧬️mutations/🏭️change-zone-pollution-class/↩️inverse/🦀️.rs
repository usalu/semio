//! ↩️ `change-zone-pollution-class` inverse.
use super::ChangeZonePollutionClass;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZonePollutionClass, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZonePollutionClass(ChangeZonePollutionClass { zone_id: payload.zone_id.clone(), new_pollution_class: z.pollution_class.clone() })]
}
