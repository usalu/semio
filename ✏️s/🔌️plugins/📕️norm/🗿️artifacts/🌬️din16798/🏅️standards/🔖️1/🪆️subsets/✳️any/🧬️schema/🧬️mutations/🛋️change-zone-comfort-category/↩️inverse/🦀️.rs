//! ↩️ `change-zone-comfort-category` inverse.
use super::ChangeZoneComfortCategory;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneComfortCategory, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneComfortCategory(ChangeZoneComfortCategory { zone_id: payload.zone_id.clone(), new_comfort_category: z.comfort_category.clone() })]
}
