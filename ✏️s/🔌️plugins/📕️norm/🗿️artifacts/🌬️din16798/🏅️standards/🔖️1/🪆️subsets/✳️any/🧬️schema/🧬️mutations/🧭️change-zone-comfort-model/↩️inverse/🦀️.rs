//! ↩️ `change-zone-comfort-model` inverse.
use super::ChangeZoneComfortModel;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneComfortModel, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneComfortModel(ChangeZoneComfortModel { zone_id: payload.zone_id.clone(), new_comfort_model: z.comfort_model.clone() })]
}
